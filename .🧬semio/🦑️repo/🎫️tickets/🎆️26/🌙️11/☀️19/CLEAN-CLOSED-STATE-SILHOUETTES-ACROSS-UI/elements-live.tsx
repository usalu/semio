import * as React from "react";
import { createRoot } from "react-dom/client";
import { UiI18nProvider, UiMobileProvider, createShellI18nInstance } from "@semio-tech/ui-react";
import { compositions } from "./elements-compose.tsx";

const modules = import.meta.glob("../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/*/📖️stories/🧪️.story.tsx", { eager: true }) as Record<string, { default: { component?: React.ComponentType<never>; args?: object; decorators?: unknown[] }; [k: string]: unknown }>;

type Entry = { element: string; story: string; render: () => React.ReactNode };
const entries: Entry[] = [];
for (const [path, mod] of Object.entries(modules)) {
  const element = path.split("/").at(-3)!.replace(/^\P{L}+/u, "");
  const meta = mod.default ?? {};
  for (const [name, value] of Object.entries(mod)) {
    if (name === "default" || !value || typeof value !== "object") continue;
    const story = value as { args?: object; render?: (a: object) => React.ReactNode };
    const render = story.render ?? (meta as { render?: (a: object) => React.ReactNode }).render;
    const args = { ...(meta.args ?? {}), ...(story.args ?? {}) };
    const Component = meta.component as React.ComponentType<object> | undefined;
    entries.push({ element, story: name, render: () => render ? <Hosted render={render} args={args} /> : Component ? <Component {...args} /> : null });
  }
}

function Hosted({ render, args }: { render: (a: object) => React.ReactNode; args: object }) { return <>{render(args)}</>; }

for (const [name, render] of Object.entries(compositions)) entries.push({ element: "Compose", story: name, render });

class Boundary extends React.Component<{ children: React.ReactNode }, { error?: string }> {
  state: { error?: string } = {};
  static getDerivedStateFromError(e: Error) { return { error: String(e.message ?? e) }; }
  render() { return this.state.error ? <pre data-render-error>{this.state.error}</pre> : this.props.children; }
}

function Story({ entry }: { entry: Entry }) {
  const C = () => <>{entry.render()}</>;
  return <Boundary><div style={{ padding: 24, minHeight: 200, height: "100vh", boxSizing: "border-box" }}><C /></div></Boundary>;
}

const requestedLocale = new URLSearchParams(location.search).get("locale");
const locale = requestedLocale === "en" || requestedLocale === "de" ? requestedLocale : undefined;
const i18n = locale && createShellI18nInstance(locale);
if (locale) document.documentElement.lang = locale;

let root: ReturnType<typeof createRoot> | undefined;
let host: HTMLDivElement | undefined;
function mount(node: React.ReactNode) {
  root?.unmount(); host?.remove();
  for (const el of document.body.children) if (el instanceof HTMLElement && el.id === "root") el.style.display = "none";
  host = document.createElement("div"); host.id = "round3-live"; host.style.cssText = "position:fixed;inset:0;overflow:auto;background:var(--background,#fff);z-index:1";
  document.body.append(host);
  root = createRoot(host);
  root.render(i18n ? <UiI18nProvider instance={i18n}><UiMobileProvider mobile={innerWidth < 768}>{node}</UiMobileProvider></UiI18nProvider> : node);
}
export const list = () => entries.map(e => `${e.element}/${e.story}`);
export function show(key: string) {
  const entry = entries.find(e => `${e.element}/${e.story}` === key)!;
  mount(<Story entry={entry} />);
}
export function showElement(element: string) {
  mount(<>{entries.filter(e => e.element === element).map(e => <Story key={e.story} entry={e} />)}</>);
}
(window as unknown as { round3: unknown }).round3 = { list, show, showElement, mount, React };

type Probe = { probe: (o?: object) => { count: number; pending?: boolean; overlaps?: unknown[]; violations: { distance: number; a: string; b: string }[] } };
const wait = (ms: number) => new Promise(r => setTimeout(r, ms));
const frame = () => new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)));
let cached: Probe | undefined;
async function loadProbe() { return cached ??= await import(/* @vite-ignore */ "/@fs" + import.meta.url.replace(/^.*?\/@fs/, "").replace(/\/round3-elements-live\.tsx.*$/, "") + "/border-gap-probe.js" + "?v=1") as Probe; }
async function sweep(keys: string[], settle = 350) {
  const p = await loadProbe();
  const out: object[] = [];
  for (const k of keys) {
    try { show(k); await frame(); await wait(settle); const r = p.probe(); if (r.count || r.overlaps?.length || r.pending) out.push({ k, c: r.count, pend: r.pending, ov: r.overlaps?.length, v: r.violations.slice(0, 3).map(v => `${v.distance} ${v.a} || ${v.b}`.slice(0, 260)) }); } catch (e) { out.push({ k, err: String(e).slice(0, 100) }); }
  }
  return out;
}
const frames = () => [...document.querySelectorAll("#round3-live [data-shared-border-frame]")].map(e => { const x = e.getBoundingClientRect(); return [(e as HTMLElement).dataset.slot, x.left.toFixed(1), x.top.toFixed(1), x.right.toFixed(1), x.bottom.toFixed(1), e.getAttribute("data-shared-border-attached")].join(" "); });
Object.assign((window as unknown as { round3: object }).round3, { sweep, frames, loadProbe, wait });
