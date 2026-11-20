/** 🧭️ Browser-only resize, rotate, and cancellation checks for the live Layout gumball. */
import assert from "node:assert/strict";
import { waitForProjection } from "./browser-projection.mjs";

const moveAxisLength = 56;
const rotationRadius = 44;
const uniformKnobOffset = rotationRadius * 0.7;
const inspectorFields = ["x", "y", "width", "height", "rotation"];

const finite = (value, field) => {
  assert.ok(Number.isFinite(value), field + " is a finite number");
  return value;
};

const readChrome = page => page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection.dumpChrome()));

function gestureRows(chrome, cursor, verb) {
  return chrome.actions.filter(row => row.seq > cursor && row.action === verb).map(row => {
    assert.equal(row.origin, "gesture", "the renderer dispatched " + verb + " through its gesture path");
    assert.equal(typeof row.args, "string", verb + " carries native arguments");
    return JSON.parse(row.args);
  });
}

function terminalMoves(payloads, phase, frameId) {
  assert.ok(payloads.length >= 2, "a streamed gesture has a tick and a terminal action");
  assert.equal(payloads.at(-1).phase, phase);
  assert.ok(payloads.slice(0, -1).every(payload => payload.phase === "stream"), "only stream ticks precede the terminal action");
  for (const payload of payloads) assert.deepEqual(payload.ids, [frameId], "every action carries the same selected frame");
  return payloads;
}

function geometryMatches(actual, expected, epsilon) {
  return inspectorFields.every(key => Math.abs(finite(actual[key], key) - expected[key]) <= epsilon);
}

/** 📐️ Computes the Inspector geometry from the native trace for one selected frame. */
export function expectedTransformedGeometry(kind, before, ticks) {
  if (kind === "cancel") return { ...before };
  if (kind === "rotate") {
    const angle = ticks.reduce((sum, tick) => sum + finite(tick.angle, "rotation delta"), 0);
    return { ...before, rotation: before.rotation + angle };
  }
  assert.equal(kind, "resize", "recognized geometry operation");
  const sx = ticks.reduce((product, tick) => product * finite(tick.sx, "X scale factor"), 1);
  const sy = ticks.reduce((product, tick) => product * finite(tick.sy, "Y scale factor"), 1);
  assert.ok(sx > 0 && sy > 0, "scale factors stay positive");
  const width = before.width * sx;
  const height = before.height * sy;
  return { ...before, x: before.x + (before.width - width) / 2, y: before.y + (before.height - height) / 2, width, height };
}

async function performGesture(page, frameId, verb, from, to, terminal, timeout) {
  const chrome = await readChrome(page);
  assert.equal(chrome.armed, true, "the live Transform utility is armed");
  const cursor = chrome.actions.at(-1)?.seq ?? 0;
  const actions = async () => gestureRows(await readChrome(page), cursor, verb);
  const wait = phase => waitForProjection(verb + " " + phase, actions, rows => rows.some(row => row.phase === phase), { timeout, pause: delay => page.waitForTimeout(delay) });

  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  try {
    await page.mouse.move(to.x, to.y, { steps: 8 });
    await wait("stream");
    if (terminal === "abort") {
      await page.keyboard.press("Escape");
      await wait("abort");
    }
  } finally {
    await page.mouse.up();
  }
  const movements = terminalMoves(await wait(terminal), terminal, frameId);
  return { movements, actions };
}

/** 🖱️ Drives each gesture with trusted browser input and checks the real Inspector after it settles. */
export async function verifyResizeRotateCancel({ page, frameId, openGeometry, closeGeometry, locateHorizontal, spec }) {
  const verify = async (kind, verb, position, terminal) => {
    const before = await openGeometry();
    await closeGeometry();
    const horizontal = await locateHorizontal();
    const pivot = { x: horizontal.x - moveAxisLength, y: horizontal.y };
    const [from, to] = position(pivot, horizontal);
    const { movements, actions } = await performGesture(page, frameId, verb, from, to, terminal, spec.timeoutMs);
    const expected = expectedTransformedGeometry(kind, before, movements);
    if (kind === "resize") {
      assert.ok(movements.every(row => row.sx > 0 && row.sy > 0 && Math.abs(row.sx - row.sy) <= spec.epsilon), "the uniform knob dispatches positive uniform factors");
      assert.ok(expected.width > before.width + spec.epsilon && expected.height > before.height + spec.epsilon, "the outward resize increases both dimensions");
    } else if (kind === "rotate") {
      assert.ok(Math.abs(expected.rotation - before.rotation) > spec.epsilon, "the ring changes the frame angle");
    } else {
      assert.ok(movements.slice(0, -1).some(row => Number.isFinite(row.dx) && Math.abs(row.dx) > spec.epsilon), "cancellation follows a measured preview movement");
      assert.ok(typeof movements.at(-1).reason === "string" && movements.at(-1).reason.length > 0, "Escape records a native abort reason");
    }

    const after = await waitForProjection(kind + " Inspector geometry", openGeometry, geometry => geometryMatches(geometry, expected, spec.epsilon), {
      timeout: spec.timeoutMs,
      pause: delay => page.waitForTimeout(delay),
    });
    await closeGeometry();
    terminalMoves(await actions(), terminal, frameId);
    return { before, expected, after, from, to, movements };
  };

  const resize = await verify("resize", "scaleSelection", pivot => {
    const from = { x: pivot.x + uniformKnobOffset, y: pivot.y - uniformKnobOffset };
    return [from, { x: from.x + spec.scaleCssPixels, y: from.y - spec.scaleCssPixels }];
  }, "commit");
  const rotate = await verify("rotate", "rotateSelection", pivot => [
    { x: pivot.x, y: pivot.y + rotationRadius },
    { x: pivot.x - rotationRadius * Math.sin(spec.rotateRadians), y: pivot.y + rotationRadius * Math.cos(spec.rotateRadians) },
  ], "commit");
  const cancel = await verify("cancel", "translateSelection", (_pivot, horizontal) => [
    horizontal, { x: horizontal.x + spec.cancelCssPixels, y: horizontal.y },
  ], "abort");

  return { valid: true, frameId, resize, rotate, cancel };
}
