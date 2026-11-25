const t=(await page.locator("body").innerText()).replace(/\n+/g," | ");
const cap=(await page.getByRole("tabpanel",{name:"Results"}).locator("p").first().innerText().catch(()=>"?")).trim();
const nodes=await page.evaluate(()=>{const b=[...document.querySelectorAll("[role=button]")].map(b=>b.textContent.trim()).filter(x=>/^(Nodes|Elements|Solids) \(\d+\)/.test(x));return b;});
return {cap,nodes,unsaved:/Unsaved/.test(t),notice:t.match(/Opened[^|]*/)?.[0],hx:globalThis.__hx.splice(0)};
