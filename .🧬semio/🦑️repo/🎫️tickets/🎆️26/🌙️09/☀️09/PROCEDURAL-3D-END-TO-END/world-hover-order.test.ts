import { expect, test } from "bun:test";
import { BehaviorSubject, Subject, withLatestFrom } from "rxjs";
const fixture = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/hover-publication-order.json").json();
for (const row of fixture) test(row.id, () => {
  type Mark = [string, string | null, string | null];
  const latest = new BehaviorSubject<Mark | undefined>(undefined);
  const settled = new Subject<void>();
  const published: Mark[] = [];
  let domain = "components", busy = false, owner: Mark | undefined;
  let lastEvent: string | undefined;
  function next(mark: Mark | undefined) {
    if (!mark || busy) return;
    latest.next(undefined);
    const key = JSON.stringify(mark);
    if (lastEvent === key) return;
    lastEvent = key;
    published.push(mark);
    busy = true;
  }
  const send = (mark: Mark) => {
    owner = mark[2] === null ? undefined : mark;
    latest.next(mark);
    next(mark);
  };
  settled.pipe(withLatestFrom(latest)).subscribe(([, owed]) => { busy = false; next(owed); });
  for (const [step, id] of row.steps) {
    if (step === "face") send([domain,"face","shape@mesh#0.face." + id]);
    if (step === "instance" || step === "other") send([domain,"object",(step === "other" ? "other" : "shape") + "@mesh#0"]);
    if (step === "leave" && owner) send([domain,null,null]);
    if (step === "settle") settled.next();
    if (step === "newGeometry" || step === "newDomain" || step === "unmount") {
      latest.next(undefined);
      if (step !== "newGeometry") owner = undefined;
      lastEvent = undefined;
      if (step === "newDomain") { busy = false; domain = "graph"; }
    }
  }
  expect(published).toEqual(row.events);
});
