import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { createRequire, isBuiltin, registerHooks, stripTypeScriptTypes } from "node:module";

const scratch = new URL("./🗑️generated/w/", import.meta.url);
const require = createRequire(new URL("package.json", scratch));
const hooks = registerHooks({
  resolve(specifier, context, nextResolve) {
    if (context.parentURL?.startsWith(scratch.href) && /^[@a-z]/iu.test(specifier) && !isBuiltin(specifier)) {
      const name = specifier.split("/").slice(0, specifier.startsWith("@") ? 2 : 1).join("/");
      if (!existsSync(new URL(`node_modules/${name}`, scratch))) throw Object.assign(new Error(`Module ${name} is absent from the isolated check`), { code: "MODULE_NOT_FOUND" });
    }
    return nextResolve(specifier, context);
  },
});
console.info("[DEBUG] loading isolated DOM and Lodash");
const { JSDOM } = require("jsdom");
const clamp = require("lodash/clamp");
hooks.deregister();
const course = new URL("../../../../../../../🧰️framework/🛍️products/📚️course/", import.meta.url);
const unit = new URL("🎯️targets/⚛️react/🔨️modules/🤏️drag/🟦️.ts", course);
const cases = JSON.parse(await readFile(new URL("🧪️tests/🖱️pointer-and-focus/🔣️.json", course), "utf8"));
const js = stripTypeScriptTypes(await readFile(unit, "utf8"));
const { startPointerDrag } = await import("data:text/javascript;base64," + Buffer.from(js).toString("base64"));

for (const test of cases.edgeScroll) {
  const { window } = new JSDOM('<div class="course-app"><div id="quiz-page"><div data-course-drag><span id="grip"></span><input id="copy-id"></div><div id="page"></div><div data-course-drop="seen"></div><div data-course-drop="next"></div></div></div>');
  const { document } = window;
  const host = document.querySelector(".course-app");
  const port = test.inner ? document.getElementById("quiz-page") : host;
  if (test.inner) port.style.overflowY = "auto";
  const grip = document.getElementById("grip");
  let frame;
  const drops = [];
  Object.defineProperties(port, { clientHeight: { value: 400 }, scrollHeight: { value: 1200 } });
  port.getBoundingClientRect = () => ({ top: 0, bottom: 400 });
  port.scrollTop = test.from;
  port.scrollTo = ({ top }) => { port.scrollTop = clamp(top, 0, 800); };
  document.elementFromPoint = () => host.querySelector(`[data-course-drop="${port.scrollTop === test.from ? "seen" : "next"}"]`);
  window.requestAnimationFrame = (callback) => { frame = callback; return 1; };
  window.cancelAnimationFrame = () => { frame = undefined; };
  const dispatch = (type, pointerId = 1) => grip.dispatchEvent(Object.assign(new window.MouseEvent(type, { bubbles: true, clientX: 30, clientY: test.pointer }), { pointerId, pointerType: "touch" }));
  const globals = Object.getOwnPropertyDescriptors(globalThis);
  Object.assign(globalThis, { window, document });
  try {
    const abandon = startPointerDrag({ button: 0, clientX: 30, clientY: 200, pointerId: 1, pointerType: "touch", currentTarget: grip, preventDefault() {} }, (zone) => drops.push(zone));
    for (const touch of cases.touchMove) {
      const event = new window.Event("touchmove", { bubbles: true, cancelable: true });
      document.getElementById(touch.target).dispatchEvent(event);
      assert.equal(event.defaultPrevented, touch.prevented);
    }
    dispatch("pointermove");
    assert.equal(host.querySelector(".course-drag-ghost")?.hasAttribute("inert"), true);
    assert.equal(host.querySelectorAll("#copy-id").length, 1);
    dispatch("pointerup", 2);
    assert.deepEqual(drops, []);
    const run = frame;
    frame = undefined;
    run?.(16);
    const shift = Math.sign(port.scrollTop - test.from);
    assert.equal(shift === 1 ? "down" : shift === -1 ? "up" : "none", test.scrolls);
    const target = shift === 0 ? "seen" : "next";
    assert.equal(host.querySelector(".course-drop-active")?.dataset.courseDrop, target);
    dispatch("pointerup");
    assert.deepEqual(drops, [target]);
    assert.equal(frame, undefined);
    assert.equal(host.querySelector(".course-drag-ghost"), null);
    abandon();
    const after = new window.Event("touchmove", { bubbles: true, cancelable: true });
    assert.equal(grip.dispatchEvent(after), true);
  } finally {
    for (const name of ["window", "document"]) {
      if (globals[name]) Object.defineProperty(globalThis, name, globals[name]);
      else delete globalThis[name];
    }
    window.close();
  }
}

console.info("[DEBUG] course touch drag passed five edge and two scope cases against jsdom and Lodash");
