import { expect, test } from "bun:test";
import { Subject, map, distinctUntilChanged } from "rxjs";

const fixture = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/hovered-instances.json").json();
test("authored hover owner handoffs", () => {
  type Selection = {hoveredId: string | null; hoveredIds: string[]; hoverSourceWindowId?: string | null};
  type Local = string | null | undefined;
  let local: Local;
  let authored: string[] = [];
  const scene = new Subject<Selection>();
  const ownership = scene.pipe(map(row => [row.hoverSourceWindowId, [...row.hoveredIds].sort()] as const),
    distinctUntilChanged((left,right) => JSON.stringify(left) === JSON.stringify(right)));
  ownership.subscribe(([source, ids]) => {
    authored = ids;
    if (source !== undefined && source !== "preview:1") local = undefined;
  });
  for (const step of fixture.handoff) {
    if (step.selection) scene.next(step.selection);
    if ("local" in step) local = step.local;
    const ids = local === undefined ? authored : local === null ? [] : [local];
    expect(fixture.instances.filter(instance => ids.includes(instance.id)).map(instance => instance.id)).toEqual(step.expected);
  }
});

test("paints only the instances sharing each authored local publication", async () => {
  const { from, groupBy, mergeMap, toArray, lastValueFrom } = await import("rxjs");
  const fixture = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🎯️grouped-hover.json").json();
  const cases = fixture.cases.filter(row => row.expectedPaint);
  const cohorts = await lastValueFrom(from(cases).pipe(
    groupBy(row => JSON.stringify(row.expected)),
    mergeMap(group => group.pipe(toArray())),
    toArray(),
  ));
  for (const rows of cohorts) {
    const painted = rows.map(row => row.instance.id);
    for (const row of rows) expect(painted).toEqual(row.expectedPaint);
  }
});
