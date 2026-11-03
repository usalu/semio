/** 🎯️ Keyboard focus of the grips, TypeScript twin of `./🦀️.rs` and of the grip keys of `NormShellHooks::handle_key`: which grip of the open artifact has the keyboard, and how Tab, Shift+Tab and Ctrl/Cmd+Arrow move it. The state is ephemeral and local to this user (never persisted, never shared). Enter starts the walk; with a grip focused a plain arrow nudges it by the schema step (Alt fine, Shift large), Enter opens its value field and applies it, Escape cancels and returns the keyboard to the viewport.
 *
 * 🧫️ Replayed on `🧪️tests/🧫️fixtures/🎯️grip-keyboard/🔣️.json` (`walks`, `registry`, `keys`) by both implementations. The chord table is the grip subset of `shortcuts()`: chords outside it are answered `ignored`. */

/** 🕹️ One grip the walk can reach: its handle id and the id of the entity that owns it. */
export type GripRefV1 = Readonly<{ id: string; entity: string }>;

/** 🚶️ How a key moves the grip focus: the whole selection in order (Tab, ends leave the walk) or only the grips of the focused grip's entity (Ctrl+Arrow, wraps around). */
export type GripWalkV1 = "forward" | "backward" | "forwardInEntity" | "backwardInEntity";

/** 🚶️ The outcome of a walk: the grip that takes the focus, `leave` when Tab ran past either end, `nothing` when there are no grips. */
export type GripStepV1 = Readonly<{ kind: "to"; id: string }> | Readonly<{ kind: "leave" }> | Readonly<{ kind: "nothing" }>;

/** 🚶️ The next focus. Without a current grip Tab and Shift+Tab start at the first or last grip and Ctrl+Arrow at the first or last grip of the first entity; a current grip that is no longer among the grips counts as none. */
export function walkGripsV1(grips: readonly GripRefV1[], current: string | null, how: GripWalkV1): GripStepV1 {
  if (grips.length === 0) return { kind: "nothing" };
  const forward = how === "forward" || how === "forwardInEntity";
  const at = current === null ? -1 : grips.findIndex((grip) => grip.id === current);
  if (how === "forward" || how === "backward") {
    if (at < 0) return { kind: "to", id: grips[forward ? 0 : grips.length - 1]!.id };
    if (forward && at + 1 < grips.length) return { kind: "to", id: grips[at + 1]!.id };
    if (!forward && at > 0) return { kind: "to", id: grips[at - 1]!.id };
    return { kind: "leave" };
  }
  const entity = at < 0 ? grips[0]!.entity : grips[at]!.entity;
  const ring = grips.filter((grip) => grip.entity === entity);
  const place = at < 0 ? -1 : ring.findIndex((grip) => grip.id === grips[at]!.id);
  const next = place >= 0 ? (forward ? (place + 1) % ring.length : (place + ring.length - 1) % ring.length) : forward ? 0 : ring.length - 1;
  return { kind: "to", id: ring[next]!.id };
}

/** 📣 The grips the viewports last drew and the grip that has the keyboard, per artifact domain. */
export class GripFocusRegistryV1 {
  private readonly grips = new Map<string, readonly GripRefV1[]>();
  private readonly focus = new Map<string, string>();

  /** 📣 Records the grips the viewport of `domain` just drew, in Tab order; a focus on a grip that is gone is dropped. */
  noteGrips(domain: string, grips: readonly GripRefV1[]): void {
    const focused = this.focus.get(domain);
    if (focused !== undefined && !grips.some((grip) => grip.id === focused)) this.focus.delete(domain);
    this.grips.set(domain, grips);
  }

  /** 🕹️ The grips last drawn for `domain`. */
  notedGrips(domain: string): readonly GripRefV1[] {
    return this.grips.get(domain) ?? [];
  }

  /** 🎯️ The grip of `domain` that has the keyboard focus, `null` outside the handle walk. */
  focusedGrip(domain: string): string | null {
    return this.focus.get(domain) ?? null;
  }

  /** 🎯️ Gives the keyboard focus to a grip of `domain`, or takes it away. */
  setFocusedGrip(domain: string, handle: string | null): void {
    if (handle === null) this.focus.delete(domain);
    else this.focus.set(domain, handle);
  }

  /** 🚶️ Walks the focus of `domain` over its noted grips and records the result. */
  walkFocus(domain: string, how: GripWalkV1): GripStepV1 {
    const step = walkGripsV1(this.notedGrips(domain), this.focusedGrip(domain), how);
    if (step.kind === "to") this.setFocusedGrip(domain, step.id);
    else if (step.kind === "leave") this.setFocusedGrip(domain, null);
    return step;
  }
}

/** ⌨️ A key press as the plugin shell hook sees it: the window id as scope, the key name the DOM spells and the modifiers. */
export type GripKeyEventV1 = Readonly<{ scope: string; key: string; ctrl: boolean; alt: boolean; shift: boolean; meta: boolean }>;

/** ⌨️ What the hook did with a key: nothing, took it, or asks the shell to run a host action by id. */
export type GripKeyOutcomeV1 = Readonly<{ type: "ignored" }> | Readonly<{ type: "consumed" }> | Readonly<{ type: "action"; action: string }>;

type ScopeV1 = "global" | "viewport" | "handle";

const ARROWS = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"] as const;
const KEBAB: Readonly<Record<string, string>> = { ArrowLeft: "arrow-left", ArrowRight: "arrow-right", ArrowUp: "arrow-up", ArrowDown: "arrow-down" };
const KNOWN_KEYS: readonly string[] = ["Enter", "Escape", "Tab", ...ARROWS];

/** 🪟️ The domain and role of a window id (`norm-en1998-viewport` is the `viewport` of `norm.en1998`); `undefined` for a window the grips do not live in. */
export function windowScopeV1(windowId: string): Readonly<{ role: "viewport"; domain: string }> | undefined {
  const parts = windowId.split("-");
  const last = parts[parts.length - 1];
  return last === "viewport" || last === "handle" || last === "handles" ? { role: "viewport", domain: parts.length >= 3 ? `norm.${parts[1]}` : "" } : undefined;
}

function scopeOf(event: GripKeyEventV1, registry: GripFocusRegistryV1): ScopeV1 {
  const window = windowScopeV1(event.scope);
  if (window !== undefined && registry.focusedGrip(window.domain) !== null) return "handle";
  return event.scope.includes("handle") ? "handle" : event.scope.includes("viewport") ? "viewport" : "global";
}

/** ⌨️ Handles one key of the grip walk like `NormShellHooks::handle_key`: Enter starts the walk, Tab and Ctrl/Cmd+Arrow move it, plain arrows nudge (`norm.handle.arrow-right.normal`, Alt `fine`, Shift `large`), Enter confirms and Escape cancels the focused grip. */
export function handleGripKeyV1(registry: GripFocusRegistryV1, event: GripKeyEventV1): GripKeyOutcomeV1 {
  const key = KNOWN_KEYS.find((known) => known.toLowerCase() === event.key.toLowerCase());
  if (key === undefined) return { type: "ignored" };
  const domain = windowScopeV1(event.scope)?.domain ?? "";
  const scope = scopeOf(event, registry);
  const ctrl = event.ctrl || event.meta;
  const { alt, shift } = event;
  if (scope === "viewport" && key === "Enter" && !ctrl && !alt && !shift) {
    if (registry.focusedGrip(domain) === null && registry.walkFocus(domain, "forward").kind !== "nothing") return { type: "consumed" };
    return { type: "action", action: "norm.shortcut.enter-handles" };
  }
  if (scope !== "handle") return { type: "ignored" };
  if (key === "Tab" && !ctrl && !alt) {
    const step = registry.walkFocus(domain, shift ? "backward" : "forward");
    return step.kind === "to" ? { type: "consumed" } : { type: "ignored" };
  }
  const arrow = (ARROWS as readonly string[]).includes(key);
  if (arrow && ctrl && !alt && !shift) {
    const forward = key === "ArrowRight" || key === "ArrowDown";
    return registry.walkFocus(domain, forward ? "forwardInEntity" : "backwardInEntity").kind === "nothing" ? { type: "ignored" } : { type: "consumed" };
  }
  if (key === "Escape" && !ctrl && !alt && !shift) {
    registry.setFocusedGrip(domain, null);
    return { type: "action", action: "norm.handle.escape.normal" };
  }
  if (key === "Enter" && !ctrl && !alt && !shift) return { type: "action", action: "norm.handle.enter.normal" };
  if (arrow && !ctrl && !(alt && shift)) return { type: "action", action: `norm.handle.${KEBAB[key]}.${alt ? "fine" : shift ? "large" : "normal"}` };
  return { type: "ignored" };
}
