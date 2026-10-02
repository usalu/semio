/** 🧪️ Ticket tool of work package G: a page that looks roughly like the quiz (header, cards, a scrolling pane, footer) with the pet layer mounted over it, for watching real layout in a real browser.
 *
 * Query parameters: `menagerie` (`sample`, the vectors of the product, or `architecture`, every species document of the
 * teaching menagerie that validates), `mode` (`still`, `calm`, `lively`), `quiet`, `seed`, `capacity`, `scale`, `theme`.
 * The page counts what the layer asks of the browser in `window.petProbe` (animation frames, timers, console output).
 */
import { StrictMode, type ReactElement } from "react";
import { createRoot } from "react-dom/client";
import { PET_MODES, speciesIssues, type Menagerie, type PetMode, type Species } from "@semio-tech/pets";
import "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css";
import { PetLayer } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx";
import { survey } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts";
import sample from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";

const documents = import.meta.glob<Species>("../../../../../../../../🎓️teaching/🏛️architecture/🐾️pets/*/🔣️.json", { eager: true, import: "default" });

function architecture(): Menagerie {
  const species = Object.values(documents).filter((document) => speciesIssues(document).length === 0);
  const ids = species.map((each) => each.id);
  return {
    schema: "semio.pets.menagerie/v1",
    id: "architecture-harness",
    title: { en: "Architecture (harness)", de: "Architektur (Prüfstand)" },
    species,
    bonds: [],
    casts: [{ scene: "home", core: ids.slice(0, 3), rotation: ids.slice(3) }],
  };
}

const probe = { frames: 0, timers: 0, console: [] as string[], species: 0 };
Object.assign(window, { petProbe: probe, petSurvey: survey });
const frame = window.requestAnimationFrame.bind(window);
window.requestAnimationFrame = (callback) => {
  probe.frames += 1;
  return frame(callback);
};
const timer = window.setTimeout.bind(window);
window.setTimeout = ((handler: TimerHandler, timeout?: number, ...rest: unknown[]) => {
  probe.timers += 1;
  return timer(handler, timeout, ...rest);
}) as typeof window.setTimeout;
for (const method of ["log", "info", "warn", "error", "debug"] as const) {
  const native = console[method].bind(console);
  console[method] = (...args: unknown[]) => {
    probe.console.push(`${method}: ${args.map(String).join(" ")}`);
    native(...args);
  };
}

const query = new URLSearchParams(location.search);
const menagerie = query.get("menagerie") === "architecture" ? architecture() : (sample.menagerie as unknown as Menagerie);
probe.species = menagerie.species.length;
const mode = (PET_MODES as readonly string[]).includes(query.get("mode") ?? "") ? (query.get("mode") as PetMode) : "calm";
const number = (name: string): number | undefined => (query.has(name) ? Number(query.get(name)) : undefined);
if (query.get("theme") === "dark") document.documentElement.dataset.theme = "dark";

function Card(props: { readonly name: string; readonly title: string; readonly action?: string }): ReactElement {
  return (
    <article className={`card ${props.name}`} data-pet-surface="">
      <h3>{props.title}</h3>
      <p>Pets stand on the top edge of this card, never in front of its words.</p>
      {props.action === undefined ? null : <button type="button">{props.action}</button>}
    </article>
  );
}

function Harness(): ReactElement {
  return (
    <div className="app">
      <header>
        <strong>Pet layer harness</strong>
        <a href="#one">One</a>
        <a href="#two">Two</a>
        <button type="button">Menu</button>
      </header>
      <main>
        <Card name="card-a" title="Card A" action="Open" />
        <Card name="card-b" title="Card B" />
        <span className="note" data-pet-keepout="">
          kept free
        </span>
        <Card name="card-c" title="Card C" action="Start" />
        <Card name="card-d" title="Card D" />
        <div className="pane" id="pane">
          <Card name="card-e" title="Card E (scrolls)" />
          <Card name="card-f" title="Card F (scrolls)" />
          <Card name="card-g" title="Card G (scrolls)" />
        </div>
      </main>
      <footer>
        <a href="#imprint">Imprint</a>
        <a href="#privacy">Privacy</a>
      </footer>
      <PetLayer menagerie={menagerie} scene="home" mode={mode} quiet={query.get("quiet") === "1"} seed={number("seed") ?? 3} capacity={number("capacity")} scale={number("scale")} />
    </div>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Harness />
  </StrictMode>,
);
