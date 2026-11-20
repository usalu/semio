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
    const [node,port]=address.split("@");
    const button=document.createElement("button");
    button.dataset.node=node;
    if(port!==undefined)button.dataset.port=port;
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
    const target=owner?{node:active.dataset.node,port:active.dataset.port}:undefined;
    const expected=row.node===undefined?undefined:{node:row.node,port:row.port};
    expect(target).toEqual(expected);
    expect(owner==="graph").toBe(Boolean(row.graphHover));
    graph.replaceChildren();
    viewport.replaceChildren();
  }
  window.close();
});
