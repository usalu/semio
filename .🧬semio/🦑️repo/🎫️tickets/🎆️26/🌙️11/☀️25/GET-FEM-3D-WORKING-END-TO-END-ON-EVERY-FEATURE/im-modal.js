const o={};
o.dialog=await page.evaluate(()=>[...document.querySelectorAll('[role=dialog],[role=alertdialog]')].map(e=>e.innerText.slice(0,500)));
o.hx=globalThis.__hx.splice(0);
return o;
