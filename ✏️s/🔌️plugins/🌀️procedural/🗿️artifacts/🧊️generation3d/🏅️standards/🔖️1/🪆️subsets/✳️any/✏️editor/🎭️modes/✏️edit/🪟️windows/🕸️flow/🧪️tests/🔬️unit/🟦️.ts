import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {JSDOM} from "jsdom";

test("a local graph hover owns projection while the viewport clears an earlier component",()=>{
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔬️unit/🔣️.json",import.meta.url),"utf8"));
  const {window}=new JSDOM('<main><section id="graph"></section><section id="viewport"></section></main>');
  const document=window.document;
  const graph=document.getElementById("graph")!;
  const viewport=document.getElementById("viewport")!;
  const control=(parent:HTMLElement,address:string,role:string)=>{
    const graphTarget = parent === graph;
    const value = graphTarget ? address.slice(2) : address;
    const separator = value.lastIndexOf("@");
    const button=document.createElement("button");
    if (graphTarget && address.startsWith("e:")) button.dataset.edge = value;
    else {
      button.dataset.node = separator < 0 ? value : value.slice(0, separator);
      if (separator >= 0) button.dataset.port = decodeURIComponent(value.slice(separator + 1));
    }
    button.dataset.kind=role;
    parent.append(button);
    return button;
  };
  for(const row of fixture.expected.componentHovers){
    const [source,qualified]=row.id.split("#");
    const [,role]=qualified.split(".");
    const component=control(viewport,source,role);
    component.focus();
    if(row.graphHover)control(graph,row.graphHover,"graph").focus();
    const active=document.activeElement as HTMLButtonElement;
    const owner=graph.contains(active)?"graph":viewport.contains(active)&&active.dataset.kind===row.mode?"component":null;
    expect(owner ? active.dataset.node : undefined).toBe(row.node);
    expect(owner ? active.dataset.port : undefined).toBe(row.port);
    expect(owner ? active.dataset.edge : undefined).toBe(row.edge);
    expect(owner==="graph").toBe(Boolean(row.graphHover));
    graph.replaceChildren();
    viewport.replaceChildren();
  }
  window.close();
});
