const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const {test} = require("node:test");
const {JSDOM} = require("jsdom");
const cases = require("./matrix-settle.json");

const code = fs.readFileSync(path.join(__dirname,"silhouette-probe.js"),"utf8");
for (const scenario of cases) test(scenario.name, async () => {
  const dom = new JSDOM('<section id="owner"><button>Test</button></section><section data-shared-border-root data-shared-border-pending></section>', {runScripts:"outside-only"});
  const {window} = dom;
  const owner = window.document.getElementById("owner");
  const target = owner.querySelector("button");
  owner.toggleAttribute("data-shared-border-root",scenario.ownerAttached);
  owner.toggleAttribute("data-shared-border-pending",scenario.ownerPending);
  target.getBoundingClientRect = () => new window.DOMRect(10,10,40,20);
  window.PointerEvent = window.MouseEvent;
  let frames = 0;
  window.requestAnimationFrame = callback => {
    queueMicrotask(()=>{
      frames++;
      if (frames === scenario.pendingFrame) owner.setAttribute("data-shared-border-pending","");
      if (frames === scenario.readyFrame) {
        owner.setAttribute("data-shared-border-root","");
        owner.removeAttribute("data-shared-border-pending");
      }
      callback(frames);
    });
    return frames;
  };
  try {
    window.eval(code);
    let modes = 0;
    window.silhouette.check = () => (modes++,{issues:[]});
    const result = await window.silhouette.sweep(owner);
    assert.equal(result.targets,1);
    assert.equal(modes,2);
    assert.equal(frames,scenario.frames);
  } finally {
    window.close();
  }
});
