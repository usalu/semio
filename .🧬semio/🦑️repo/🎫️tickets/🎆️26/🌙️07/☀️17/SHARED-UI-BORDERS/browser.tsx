import React from "react";
import { createRoot } from "react-dom/client";
import { WindowChrome } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/🟦️.tsx";
import { ButtonGroup, ButtonGroupItem } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx";
import { Popover, PopoverContent, PopoverTrigger } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️.tsx";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx";
import { createShellScope, ShellScopeProvider } from "/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx";
import { createMemoryStoragePort } from "/🧰️framework/🔨️modules/🖱️ui/🔨️modules/🗄️storage-port/🟦️.ts";
import "/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css";

function Frame({ id, left, active = false }: { id: string; left: number; active?: boolean }) {
  return <WindowChrome active={active} style={{ position: "absolute", left, top: 80, width: 300, height: 240 }} titleChips={<div style={{ width: 120, height: 32 }}>{id}</div>} body={<div style={{ height: 208 }}>
    <ButtonGroup id={`${id}-buttons`} style={{ width: 160, height: 32 }}><ButtonGroupItem text="One" /><ButtonGroupItem text="Two" /></ButtonGroup>
    <Popover><PopoverTrigger id={`${id}-menu`}>Open Menu</PopoverTrigger><PopoverContent data-probe={`${id}-popup`} style={{ width: 400, height: 160 }}>Menu Content</PopoverContent></Popover>
    <Select defaultValue="a"><SelectTrigger id={`${id}-select`}><SelectValue /></SelectTrigger><SelectContent><SelectItem value="a">Alpha</SelectItem><SelectItem value="b">Beta</SelectItem></SelectContent></Select>
  </div>} />;
}

function Fixture() {
  const [scope] = React.useState(() => createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en" }));
  return <ShellScopeProvider scope={scope}><main ref={node => { scope.rootRef.current = node; }} className="semio-scope" style={{ position: "relative", width: 900, height: 600, background: "#eee" }}>
    <div id="nav" data-shared-border-frame="" data-shared-border-edges="bottom" style={{ position: "absolute", left: 20, top: 48, width: 600, height: 32, borderBottom: "1px solid" }}>Navigation</div>
    {new URLSearchParams(location.search).has("markers-only") ? <>
      <ButtonGroup id="standalone-left" style={{ position: "absolute", left: 20, top: 80, width: 160, height: 32 }}><ButtonGroupItem text="First" /></ButtonGroup>
      <ButtonGroup id="standalone-right" style={{ position: "absolute", left: 180, top: 80, width: 160, height: 32 }}><ButtonGroupItem text="Second" /></ButtonGroup>
    </> : <><Frame id="Left" left={20} active /><Frame id="Right" left={320} /></>}
    <div id="portal-layer" ref={node => { scope.portalLayerRef.current = node; }} style={{ position: "fixed", inset: 0, pointerEvents: "none", zIndex: 100 }} />
  </main></ShellScopeProvider>;
}

createRoot(document.getElementById("root")!).render(<Fixture />);
