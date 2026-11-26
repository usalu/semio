/** 🔍️ Reserve a separate eight-MiB shadow stack during the frozen core diagnostic. */
import assert from "node:assert/strict";

const instantiate = WebAssembly.instantiate.bind(WebAssembly);
let corrected = false;
WebAssembly.instantiate = async (...args) => {
  const instance = await instantiate(...args);
  const exports = instance.exports ?? instance.instance.exports;
  const stack = exports.__semio_diagnostic_stack;
  if (stack) {
    assert.equal(corrected, false);
    assert.equal(stack.value, 1048576);
    assert(exports.memory instanceof WebAssembly.Memory);
    const oldBytes = exports.memory.buffer.byteLength;
    const stackPages = 128;
    assert.equal(exports.memory.grow(stackPages), oldBytes / 65536);
    stack.value = oldBytes + stackPages * 65536;
    corrected = true;
    console.error(`[DEBUG] Reserved synthetic frozen core stack ${oldBytes}..${stack.value}`);
  }
  return instance;
};
