/** 👆️ The swipe phase of LayeredOverview: the touch drive, the scroll and pointer-capture helpers and the swipe hook. */
import { type LayeredOffset, type LayeredCell, type LayeredAxis, type LayeredLanding, flingStep, cellOffset, nearestCell, swipeAxis, swipeDragOffset, swipeStep, LAYERED_WHEEL_QUIET_MS, LAYERED_WHEEL_STEP_PX } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import * as React from "react";

/** 🕹️ What the swipe gestures move: the strip's offset now, the occupied cells, a drag that follows the finger, a settle onto a cell, a stop. */
export interface LayeredSwipeDrive {
  readonly offset: () => LayeredOffset;
  readonly cells: () => readonly LayeredCell[];
  readonly drag: (offset: LayeredOffset) => void;
  readonly aim: (from: LayeredCell, axis: LayeredAxis, step: number) => LayeredLanding | null;
  readonly settle: (to: LayeredOffset) => void;
  readonly finish: () => void;
  readonly stop: () => void;
}

/** 🍩️ The page that comes in across an edge of the wrapping strip: shifted from its `cell` to the `slot` beside the view until the strip rests. */
export interface LayeredGhost extends LayeredLanding {
  readonly id: string;
}

/** 🖐️ One touch on the overview: where it went down, the cell the strip left, the axis it locked to and, when it scrolls a tall card instead,
 * that card's scroller and its position at the lock. */
interface LayeredTouch {
  readonly id: number;
  readonly x: number;
  readonly y: number;
  readonly target: EventTarget | null;
  readonly samples: { t: number; x: number; y: number }[];
  start: LayeredOffset;
  from: LayeredCell;
  axis: LayeredAxis | null;
  scroller: HTMLElement | null;
  scrolled: number;
  aimed: number;
}

const VELOCITY_WINDOW_MS = 100;
const CLICK_AFTER_SWIPE_MS = 400;
const along = (axis: LayeredAxis, x: number, y: number): number => (axis === "x" ? x : y);
const scrollPosition = (element: HTMLElement, axis: LayeredAxis): number => (axis === "x" ? element.scrollLeft : element.scrollTop);
const scrollAlong = (element: HTMLElement, axis: LayeredAxis, value: number): void => {
  if (axis === "x") element.scrollLeft = value;
  else element.scrollTop = value;
};

/** 📜️ The nearest element from `target` up to (not including) `root` that scrolls along `axis` and still can toward `direction` (+1 forward,
 * −1 back): a tall card takes the gesture before the strip does, as a nested scroller does natively. */
function scrollerToward(target: EventTarget | null, root: HTMLElement, axis: LayeredAxis, direction: number): HTMLElement | null {
  for (let element = target instanceof Element ? target : null; element !== null && element !== root; element = element.parentElement) {
    if (!(element instanceof HTMLElement)) continue;
    const style = getComputedStyle(element);
    if (!/auto|scroll/u.test(axis === "x" ? style.overflowX : style.overflowY)) continue;
    const [position, size, view] = axis === "x" ? [element.scrollLeft, element.scrollWidth, element.clientWidth] : [element.scrollTop, element.scrollHeight, element.clientHeight];
    if (direction > 0 ? position + view < size - 1 : position > 0) return element;
  }
  return null;
}

/** 🧲️ Captures `pointerId` on `element` where the browser can: an environment without pointer capture is left alone, and a pointer that
 * is no longer active cannot be captured. */
function capturePointer(element: HTMLElement, pointerId: number): void {
  if (element.hasPointerCapture?.(pointerId) !== false) return;
  try {
    element.setPointerCapture(pointerId);
  } catch (error) {
    if (!(error instanceof DOMException)) throw error;
  }
}

/** 🏎️ A touch's speed along `axis` over its last {@link VELOCITY_WINDOW_MS} (px/ms). */
function touchVelocity(touch: LayeredTouch, axis: LayeredAxis): number {
  const last = touch.samples[touch.samples.length - 1]!;
  const first = touch.samples.find((sample) => last.t - sample.t <= VELOCITY_WINDOW_MS) ?? last;
  const elapsed = last.t - first.t;
  return elapsed > 0 ? (along(axis, last.x, last.y) - along(axis, first.x, first.y)) / elapsed : 0;
}

/** ↔️ Swiping the strip while `enabled`: a touch or pen that travels past the slop locks to one axis and carries the strip along it toward the
 * page that comes in on that side — across an edge the page from the far side, shifted beside the view ({@link swipeWrapTarget}); a rubber band
 * only where the row or column holds no other page — and its release settles there or back ({@link swipeStep}); a touch that starts on a tall card
 * scrolls the card while it can and flings it on release; once a touch is a swipe the root captures its pointer, so the rest of the gesture
 * reaches it even when the element it started on re-renders away; a second finger (never the primary pointer) hands the gesture to the
 * browser's pinch zoom, while a new primary touch always starts a new gesture. The wheel steps one cell per gesture along its longer axis
 * (Shift turns it sideways) once no card under it can scroll that way. The click a swipe would end in is swallowed. Mouse drags are left
 * alone. The root, every card's cell and everything inside a cell (at zero specificity, so a card's own
 * `touch-action` wins) carry `touch-action: pinch-zoom`: a browser reads `touch-action` only up to the nearest scroll container, and a cell
 * that may scroll is one, as is any scrolling part of a card, so without its own the browser would take the swipe — every scroller inside a
 * card is scrolled here instead.
 * @see https://w3c.github.io/pointerevents/#determining-supported-direct-manipulation-behavior */
export function useSwipe(rootRef: React.RefObject<HTMLDivElement | null>, enabled: boolean, drive: React.RefObject<LayeredSwipeDrive>): void {
  React.useEffect(() => {
    const root = rootRef.current;
    if (!enabled || root === null) return;
    let touch: LayeredTouch | null = null;
    let fling = 0;
    let swallowUntil = -Infinity;
    const wheel = { at: -Infinity, sum: 0, spent: false };
    const size = (axis: LayeredAxis): number => Math.max(1, axis === "x" ? root.clientWidth : root.clientHeight);
    const residual = (current: LayeredTouch, axis: LayeredAxis): number => along(axis, current.start.x - current.from.column, current.start.y - current.from.row);
    const flingScroller = (element: HTMLElement, axis: LayeredAxis, velocity: number) => {
      let speed = velocity;
      let last = performance.now();
      const tick = (now: number) => {
        const step = flingStep(speed, now - last);
        last = now;
        const before = scrollPosition(element, axis);
        scrollAlong(element, axis, before + step.distance);
        speed = step.velocity;
        if (!step.done && scrollPosition(element, axis) !== before) fling = requestAnimationFrame(tick);
      };
      fling = requestAnimationFrame(tick);
    };
    const onDown = (event: PointerEvent) => {
      if (event.pointerType === "mouse") return;
      if (touch !== null) {
        const held = touch;
        touch = null;
        if (held.axis !== null && held.scroller === null) drive.current.settle(cellOffset(held.from));
        if (!event.isPrimary) return;
      }
      cancelAnimationFrame(fling);
      drive.current.finish();
      const start = drive.current.offset();
      touch = { id: event.pointerId, x: event.clientX, y: event.clientY, target: event.target, samples: [{ t: event.timeStamp, x: event.clientX, y: event.clientY }], start, from: nearestCell(start, drive.current.cells()), axis: null, scroller: null, scrolled: 0, aimed: 0 };
    };
    const onMove = (event: PointerEvent) => {
      const current = touch;
      if (current === null || event.pointerId !== current.id) return;
      current.samples.push({ t: event.timeStamp, x: event.clientX, y: event.clientY });
      if (current.samples.length > 32) current.samples.splice(0, current.samples.length - 32);
      const dx = event.clientX - current.x;
      const dy = event.clientY - current.y;
      if (current.axis === null) {
        const axis = swipeAxis(dx, dy);
        if (axis === null) return;
        current.axis = axis;
        capturePointer(root, event.pointerId);
        current.scroller = scrollerToward(current.target, root, axis, -Math.sign(along(axis, dx, dy)));
        if (current.scroller !== null) current.scrolled = scrollPosition(current.scroller, axis);
        else {
          drive.current.stop();
          current.start = drive.current.offset();
          current.from = nearestCell(current.start, drive.current.cells());
        }
      }
      const moved = along(current.axis, dx, dy);
      if (current.scroller !== null) {
        scrollAlong(current.scroller, current.axis, current.scrolled - moved);
        return;
      }
      const shift = residual(current, current.axis) - moved / size(current.axis);
      if (Math.sign(shift) !== 0 && Math.sign(shift) !== current.aimed) {
        current.aimed = Math.sign(shift);
        drive.current.aim(current.from, current.axis, current.aimed);
      }
      drive.current.drag(swipeDragOffset(current.from, current.axis, shift, drive.current.cells()));
    };
    const onEnd = (event: PointerEvent) => {
      const current = touch;
      if (current === null || event.pointerId !== current.id) return;
      touch = null;
      if (current.axis === null) return;
      const cancelled = event.type === "pointercancel";
      if (!cancelled) swallowUntil = performance.now() + CLICK_AFTER_SWIPE_MS;
      const velocity = touchVelocity(current, current.axis);
      if (current.scroller !== null) {
        if (!cancelled) flingScroller(current.scroller, current.axis, -velocity);
        return;
      }
      const view = size(current.axis);
      const moved = along(current.axis, event.clientX - current.x, event.clientY - current.y) - residual(current, current.axis) * view;
      const step = cancelled ? 0 : swipeStep(moved, velocity, view);
      const landing = step === 0 ? null : drive.current.aim(current.from, current.axis, step);
      drive.current.settle(landing?.slot ?? cellOffset(current.from));
    };
    const onClick = (event: MouseEvent) => {
      if (performance.now() > swallowUntil) return;
      swallowUntil = -Infinity;
      event.preventDefault();
      event.stopPropagation();
    };
    const onWheel = (event: WheelEvent) => {
      if (event.ctrlKey) return;
      const scale = event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? size("y") : 1;
      const sideways = event.shiftKey && event.deltaX === 0;
      const axis: LayeredAxis = sideways || Math.abs(event.deltaX) > Math.abs(event.deltaY) ? "x" : "y";
      const delta = (sideways ? event.deltaY : along(axis, event.deltaX, event.deltaY)) * scale;
      if (delta === 0 || scrollerToward(event.target, root, axis, Math.sign(delta)) !== null) return;
      event.preventDefault();
      if (event.timeStamp - wheel.at > LAYERED_WHEEL_QUIET_MS) Object.assign(wheel, { sum: 0, spent: false });
      wheel.at = event.timeStamp;
      if (wheel.spent) return;
      wheel.sum += delta;
      if (Math.abs(wheel.sum) < LAYERED_WHEEL_STEP_PX) return;
      wheel.spent = true;
      drive.current.finish();
      const landing = drive.current.aim(nearestCell(drive.current.offset(), drive.current.cells()), axis, Math.sign(wheel.sum));
      if (landing !== null) drive.current.settle(landing.slot);
    };
    root.addEventListener("pointerdown", onDown);
    root.addEventListener("pointermove", onMove);
    root.addEventListener("pointerup", onEnd);
    root.addEventListener("pointercancel", onEnd);
    root.addEventListener("click", onClick, true);
    root.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      cancelAnimationFrame(fling);
      root.removeEventListener("pointerdown", onDown);
      root.removeEventListener("pointermove", onMove);
      root.removeEventListener("pointerup", onEnd);
      root.removeEventListener("pointercancel", onEnd);
      root.removeEventListener("click", onClick, true);
      root.removeEventListener("wheel", onWheel);
    };
  }, [rootRef, enabled, drive]);
}
