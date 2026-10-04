/** 🧪️ Ticket tool of work package C1a (second round): a page with cards, words and controls under the pet layer, for driving the learner's hand in a real browser.
 *
 * Query parameters: `mode` (`still`, `calm`, `lively`; `calm` when absent), `quiet`, `seed`, `play` (`0` switches play
 * off), `capacity`, `keepouts` (`loose`: only words are kept free, so a pet may stand in front of a button). The page notes what reached it in `window.pageRecord` (clicks and presses the page's own listeners
 * heard, by element id) and every console call in `window.petConsole`; the recorder alias (`c1a_pets_recorder.ts`)
 * notes the stage events in `window.petRecord` and the last frame in `window.petFrame`. `window.petHand` is the
 * layer's handle (`play(species, deed)`).
 */
import { StrictMode, useRef, type ReactElement } from "react";
import { createRoot } from "react-dom/client";
import { PET_MODES, type Menagerie, type PetMode } from "@semio-tech/pets";
import "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css";
import { PetLayer, type PetLayerHandle } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx";
import sample from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";

const record: string[] = [];
const spoken: string[] = [];
Object.assign(window, { pageRecord: record, petConsole: spoken });
for (const method of ["log", "info", "warn", "error", "debug"] as const) {
  const native = console[method].bind(console);
  console[method] = (...args: unknown[]) => {
    spoken.push(`${method}: ${args.map(String).join(" ")}`);
    native(...args);
  };
}
const note = (event: Event): void => {
  const target = event.target as Element | null;
  record.push(`${event.type}:${target?.closest?.("[id]")?.id ?? target?.nodeName ?? "?"}`);
};
for (const type of ["pointerdown", "mousedown", "click", "keydown"]) document.addEventListener(type, note);

const query = new URLSearchParams(location.search);
const menagerie = sample.menagerie as unknown as Menagerie;
const mode = (PET_MODES as readonly string[]).includes(query.get("mode") ?? "") ? (query.get("mode") as PetMode) : "calm";
const number = (name: string): number | undefined => (query.has(name) ? Number(query.get(name)) : undefined);

function Card(props: { readonly id: string; readonly title: string; readonly action?: string }): ReactElement {
  return (
    <article id={props.id} className={`card ${props.id}`} data-pet-surface="">
      <h3>{props.title}</h3>
      <p>Pets stand on the top edge of this card, never in front of its words.</p>
      {props.action === undefined ? null : (
        <button id={`${props.id}-action`} type="button">
          {props.action}
        </button>
      )}
    </article>
  );
}

function Harness(): ReactElement {
  const hand = useRef<PetLayerHandle>(null);
  Object.assign(window, { petHand: hand });
  return (
    <>
      <header>
        <strong>Pet hand harness</strong>
        <a id="link" href="#one">
          One
        </a>
        <button id="menu" type="button">
          Menu
        </button>
      </header>
      <main>
        <Card id="card-a" title="Card A" action="Open" />
        <Card id="card-b" title="Card B" />
        <Card id="card-c" title="Card C" action="Start" />
        <p id="words" className="words">
          These words lie beside the pets and can be selected with the pointer, from the first word to the last one.
        </p>
        <button id="covered" className="covered" type="button" hidden>
          Under
        </button>
      </main>
      <footer>
        <a id="imprint" href="#imprint">
          Imprint
        </a>
      </footer>
      <PetLayer ref={hand} menagerie={menagerie} scene="home" mode={mode} quiet={query.get("quiet") === "1"} play={query.get("play") !== "0"} seed={number("seed") ?? 5} capacity={number("capacity")} keepouts={query.get("keepouts") === "loose" ? "p, [data-pet-keepout]" : undefined} />
    </>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Harness />
  </StrictMode>,
);
