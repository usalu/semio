import * as React from "react";
import { createRoot } from "react-dom/client";
import { UiI18nProvider, createShellI18nInstance, Button, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, SharedBorderRowLabel, ToggleGroup, MenuItem, floatingPaneAsideClass, cn } from "@semio-tech/ui-react";
import { InferencePortPanel } from "../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🪟️presentation/🟦️.tsx";

const kinds = ["vertex", "edge", "wire", "face", "shell", "solid", "anchor", "object", "curve", "surface", "mesh"] as const;
const actions = ["move", "copy", "rotate", "scale", "mirror", "array"] as const;
const commands = ["box", "line", "move"] as const;
const translations = {
  en: {
    editor: "Editor", action: "Action", toggle: "Toggle actions", model: "Model definition", shape: "Shape (a)", height: "Height",
    clear: "Clear", flag: "Flag", enabled: "Enabled", show: "Primitives · Show", filter: "Primitives · Filter",
    status: "Interaction x · state idle · rev 3", response: "Last response",
    kinds: { vertex: "Vertex", edge: "Edge", wire: "Wire", face: "Face", shell: "Shell", solid: "Solid", anchor: "Anchor", object: "Object", curve: "Curve", surface: "Surface", mesh: "Mesh" },
    actions: { move: "Move", copy: "Copy", rotate: "Rotate", scale: "Scale", mirror: "Mirror", array: "Array" },
    commands: { box: "Box", line: "Line", move: "Move" },
  },
  de: {
    editor: "Editor", action: "Aktion", toggle: "Aktionen umschalten", model: "Modelldefinition", shape: "Form (a)", height: "Höhe",
    clear: "Leeren", flag: "Kennzeichnung", enabled: "Aktiviert", show: "Primitive · Anzeigen", filter: "Primitive · Filtern",
    status: "Interaktion x · im Leerlauf · Rev. 3", response: "Letzte Antwort",
    kinds: { vertex: "Eckpunkt", edge: "Kante", wire: "Kantenzug", face: "Fläche", shell: "Schale", solid: "Volumenkörper", anchor: "Anker", object: "Objekt", curve: "Kurve", surface: "Oberfläche", mesh: "Netz" },
    actions: { move: "Verschieben", copy: "Kopieren", rotate: "Drehen", scale: "Skalieren", mirror: "Spiegeln", array: "Anordnen" },
    commands: { box: "Quader", line: "Linie", move: "Verschieben" },
  },
};
const requestedLocale = new URLSearchParams(location.search).get("locale");
const locale = [requestedLocale, ...navigator.languages].map(tag => tag?.toLowerCase().split("-")[0]).find(tag => tag === "en" || tag === "de");
if (!locale) throw new Error("Open the CAD fixture with ?locale=en or ?locale=de");
document.documentElement.lang = locale;
const labels = translations[locale];
function Section({ title }: { title: string }) {
  const [on, setOn] = React.useState<string[]>(kinds.slice(0, 5));
  return (
    <SharedBorderRow orientation="vertical">
      <SharedBorderRowField className="text-foreground flex-none font-semibold"><input type="checkbox" aria-label={title} />{title}</SharedBorderRowField>
      <ToggleGroup kind="multiple" wrap aria-label={title} value={on} onValueChange={setOn} items={kinds.map((k) => ({ value: k, icon: on.includes(k) ? "check" : "square", text: labels.kinds[k] }))} />
    </SharedBorderRow>
  );
}
function Aside() {
  const [open, setOpen] = React.useState(true);
  return (
    <SharedBorderRow as="aside" orientation="vertical" data-level="pane" className={cn(floatingPaneAsideClass, "h-[560px] w-[360px]")}>
      <SharedBorderRowCell className="shrink-0"><strong className="text-sm font-semibold">{labels.editor}</strong></SharedBorderRowCell>
      <SharedBorderRow orientation="vertical" className="min-h-0 flex-1 overflow-auto [&>*]:shrink-0">
        <SharedBorderRow wrap>
          {actions.map((action, index) => <Button key={action} type="button" variant="outline" icon="arrow-right" className="h-auto px-single py-half text-xs">{String.fromCharCode(65 + index)} {labels.actions[action]}</Button>)}
        </SharedBorderRow>
        <SharedBorderRow data-level="menu">
          <Input id="cmd" type="text" placeholder={labels.action} aria-label={labels.action} className="bg-transparent" />
          <Button type="button" variant="outline" icon={open ? "chevron-up" : "chevron-down"} className="flex-none p-0 text-2xs" aria-label={labels.toggle} onClick={() => setOpen((o) => !o)} />
          {open ? (
            <div className="absolute inset-x-0 top-full z-[3]">
              <SharedBorderRow role="menu" orientation="vertical" edges={["right", "bottom", "left"]} className={floatingPaneAsideClass} data-level="menu">
                <SharedBorderRow orientation="vertical" className="max-h-40 overflow-y-auto [&>*]:shrink-0">
                  {commands.map((k) => <MenuItem key={k} data-shared-border-cell="" className="rounded-none px-single py-single"><div className="flex items-center gap-single"><kbd className="text-muted-foreground inline-block min-w-6 text-center text-2xs font-bold uppercase">{k[0]}</kbd><span>{labels.commands[k]}</span></div></MenuItem>)}
                </SharedBorderRow>
              </SharedBorderRow>
            </div>
          ) : null}
        </SharedBorderRow>
        <SharedBorderRow>
          <SharedBorderRowLabel htmlFor="md-trigger">{labels.model}</SharedBorderRowLabel>
          <Select id="md" value="a"><SelectTrigger className="h-medium w-full flex-1"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="a">{labels.shape}</SelectItem></SelectContent></Select>
        </SharedBorderRow>
        <SharedBorderRow>
          <SharedBorderRowLabel htmlFor="attr">{labels.height}</SharedBorderRowLabel>
          <Input id="attr" type="number" className="h-medium w-full" />
          <Button type="button" variant="outline" icon="x" className="flex-none h-auto px-half py-0 text-2xs">{labels.clear}</Button>
        </SharedBorderRow>
        <SharedBorderRow><SharedBorderRowLabel htmlFor="b">{labels.flag}</SharedBorderRowLabel><SharedBorderRowField><input id="b" type="checkbox" />{labels.enabled}</SharedBorderRowField></SharedBorderRow>
        <Section title={labels.show} />
        <Section title={labels.filter} />
        <SharedBorderRowCell className="text-muted-foreground block text-xs">{labels.status}</SharedBorderRowCell>
        <SharedBorderRowCell className="block text-xs"><strong>{labels.response}</strong><pre className="text-2xs">{"{}"}</pre></SharedBorderRowCell>
      </SharedBorderRow>
    </SharedBorderRow>
  );
}
function App() {
  const status = { phase: "offered", jobId: "0123456789abcdef0123456789abcdef", cursor: 4, completed: 1, total: 3, proposalHash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef", cancelRequested: false, code: null } as never;
  return (
    <div className="semio-scope" style={{ padding: 0 }}>
      <Aside />
      <InferencePortPanel status={status} locale={locale} onAction={() => {}} />
    </div>
  );
}
createRoot(document.getElementById("root")!).render(<UiI18nProvider instance={createShellI18nInstance(locale)}><App /></UiI18nProvider>);
