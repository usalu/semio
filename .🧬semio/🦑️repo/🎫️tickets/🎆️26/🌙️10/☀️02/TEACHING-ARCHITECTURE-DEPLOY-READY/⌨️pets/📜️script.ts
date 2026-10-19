import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";

const root = process.cwd();
const product = resolve(root, "🧰️framework/🛍️products/🐾️pets");
const load = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const pets = await import(pathToFileURL(resolve(product, "📦️packages/🟦️typescript/🟦️.ts")).href);
const draft = await import(pathToFileURL(resolve(product, "🔨️modules/📝️draft/🟦️.ts")).href);
const attention = await import(pathToFileURL(resolve(product, "🔨️modules/👀️attention/🟦️.ts")).href);
const fixture = load(resolve(product, "🧪️tests/🧫️fixtures/⌨️keyboard-gestures/🔣️.json"));
const sample = load(resolve(product, "🧪️tests/🧫️fixtures/⚗️chemistry-rules/🔣️.json")).menagerie;
const schema = load(resolve(product, "🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: false });
ajv.addSchema(schema);
assert.deepEqual(pets.DEEDS, schema.$defs.Deed.enum);
const i18n = resolve(root, "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🌐️i18n");
const en = (await import(pathToFileURL(resolve(i18n, "🔠️english-bundle/🟦️.ts")).href)).COURSE_BUNDLE_EN.course.preferences;
const de = (await import(pathToFileURL(resolve(i18n, "🔡️german-bundle/🟦️.ts")).href)).COURSE_BUNDLE_DE.course.preferences;
for (const name of ["Hello", "Trick", "Pet", "Toss", "Circle", "Countercircle", "Stroke", "Shake"]) {
  assert.ok(en[`pets${name}`].label.normal.length > 0 && de[`pets${name}`].label.normal.length > 0);
  assert.notEqual(en[`pets${name}`].label.normal, de[`pets${name}`].label.normal);
  for (const bundle of [en, de]) assert.ok(bundle[`pets${name}Said`].label.normal.includes("{{name}}"));
}
console.log("[DEBUG] all eight deed labels and named request announcements exist in English and German bundle data PASS");
const eventOf = (species: string, deed: string) => ({ kind: "played", species, deed });
const start = (menagerie: any, species: string, state: string, grumpy = false) => {
  const stage = pets.advance(menagerie, pets.openStage(7), [{ kind: "surveyed", width: 1280, height: 720, surfaces: [{ id: "floor", x0: 0, x1: 1280, y: 720 }], keepouts: [], walls: [], fixtures: [] }, { kind: "summoned", species: [species] }, { kind: "permitted", play: true, mischief: false }]);
  return { ...stage, actors: stage.actors.map((actor: any) => ({ ...actor, state, stateSince: 0, activity: "idle", since: 0, until: 100000, partner: null, leaving: false, feeling: grumpy ? { mood: "grumpy", intensity: 0.9, since: 0 } : pets.atRest("content", 0) })) };
};
const finish = (menagerie: any, stage: any) => stage.actors[0].activity === "trick" ? pets.advance(menagerie, stage, [{ kind: "ticked", ticks: stage.actors[0].until - stage.tick }]) : stage;
for (const vector of fixture.vectors) for (const grumpy of [false, true]) {
  const stage = start(sample, vector.species, vector.state, grumpy);
  const event = eventOf(vector.species, vector.deed);
  assert.equal(ajv.validate(schema.$id + "#/$defs/Played", event), true, JSON.stringify(ajv.errors));
  const asked = pets.advance(sample, stage, [event]);
  assert.equal(asked.actors[0].trick, vector.trick, `${vector.species}/${vector.state}/${vector.deed}`);
  assert.equal(finish(sample, asked).actors[0].state, vector.after);
  assert.deepEqual(asked.actors[0].warmth, stage.actors[0].warmth);
}
let route = start(sample, fixture.route.species, fixture.route.state);
for (const [index, deed] of fixture.route.deeds.entries()) {
  route = finish(sample, pets.advance(sample, route, [eventOf(fixture.route.species, deed)]));
  assert.equal(route.actors[0].state, fixture.route.states[index]);
}
for (const deed of fixture.deeds) {
  const stage = start(sample, "glow", "bright");
  for (const changes of fixture.blocked) {
    const blocked = { ...stage, actors: [{ ...stage.actors[0], ...changes }] };
    assert.deepEqual(pets.advance(sample, blocked, [eventOf("glow", deed)]), blocked);
  }
  for (const blocked of [{ ...stage, play: false }, { ...stage, mode: "still" }]) assert.deepEqual(pets.advance(sample, blocked, [eventOf("glow", deed)]), blocked);
}
const purring = start(sample, "mist", "fluffy");
purring.actors[0].activity = "purr";
const stroked = pets.advance(sample, purring, [eventOf("mist", "stroke")]);
assert.equal(stroked.actors[0].activity, "purr");
assert.equal(stroked.actors[0].trick, null);
console.log("[DEBUG] neutral explicit keyboard gestures, natural state route, warmth isolation and permission/actor gates PASS");
const site = resolve(root, "🎓️teaching/🏛️architecture/🐾️pets");
const ensemble = load(resolve(site, "🔣️.json"));
const menagerie = pets.assembleMenagerie(ensemble, ensemble.species.map((path: string) => load(resolve(site, path))));
const database = new Database(":memory:");
try {
  const first = database.query("select json_extract(t.value, '$.id') id from json_each(?, '$.tricks') t where exists(select 1 from json_each(t.value, '$.cues') c where c.value=?) and (json_type(t.value, '$.from') is null or exists(select 1 from json_each(t.value, '$.from') f where f.value=?)) order by cast(t.key as integer) limit 1");
  let checks = 0;
  const reached = new Set<string>();
  for (const species of menagerie.species) for (const state of species.states) for (const deed of fixture.deeds) for (const grumpy of [false, true]) {
    const oracle = first.get(JSON.stringify(species), deed, state.id) as { id: string } | null;
    const stage = start(menagerie, species.id, state.id, grumpy);
    const asked = pets.advance(menagerie, stage, [eventOf(species.id, deed)]);
    const hand = draft.draftOf(menagerie, stage);
    if (deed === "shake") attention.shaken(hand, 0, stage.tick);
    else attention.cued(hand, 0, deed, stage.tick);
    assert.deepEqual(asked.actors[0], draft.sealed(hand).actors[0], `${species.id}/${state.id}/${deed}`);
    if (deed !== "shake") assert.equal(asked.actors[0].trick, oracle?.id ?? null);
    if (oracle) reached.add(`${species.id}/${oracle.id}`);
    checks++;
  }
  const authored = menagerie.species.flatMap((species: any) => species.tricks.filter((trick: any) => trick.cues.some((cue: string) => fixture.deeds.includes(cue))).map((trick: any) => `${species.id}/${trick.id}`));
  assert.deepEqual([...reached].sort(), [...authored].sort());
  console.log(`[DEBUG] SQLite JSON selection oracle + exact hand/keyboard actor equivalence PASS: ${checks} actions, ${reached.size} authored gesture tricks, ${menagerie.species.length} species`);
  database.exec("create table edges(source text, target text)");
  const insert = database.query("insert into edges values (?, ?)");
  const reach = database.query("with recursive reach(state) as (select ? union select target from edges join reach on edges.source=reach.state) select state from reach order by state");
  let reachable = 0;
  for (const species of menagerie.species) {
    database.exec("delete from edges");
    for (const state of species.states) {
      if (state.then) insert.run(state.id, state.then);
      for (const trick of species.tricks) if (trick.cues.some((cue: string) => cue === "click" || fixture.deeds.includes(cue)) && (!trick.from || trick.from.includes(state.id))) insert.run(state.id, pets.stateAfterTrick(species, state.id, trick));
    }
    const expected = reach.all(species.states[0].id).map((row: any) => row.state);
    const counts = species.states.map((state: any) => species.tricks.filter((trick: any) => trick.cues.includes("click") && (!trick.from || trick.from.includes(state.id))).length).filter((count: number) => count > 0);
    const gcd = (a: number, b: number): number => b === 0 ? a : gcd(b, a % b);
    const period = counts.reduce((a: number, b: number) => a * b / gcd(a, b), 1);
    const queue = [{ ...start(menagerie, species.id, species.states[0].id), quiet: true }];
    const visited = new Set<string>();
    const actual = new Set<string>();
    for (let index = 0; index < queue.length; index++) {
      const stage = queue[index]!;
      const actor = stage.actors[0];
      const key = `${actor.state}/${actor.warmth.tricks % period}`;
      if (visited.has(key)) continue;
      visited.add(key);
      actual.add(actor.state);
      const click = first.get(JSON.stringify(species), "click", actor.state);
      for (const deed of [...fixture.deeds, ...(click ? ["trick"] : [])]) queue.push(finish(menagerie, pets.advance(menagerie, stage, [eventOf(species.id, deed)])));
      const state = species.states.find((state: any) => state.id === actor.state);
      if (state?.lasts) queue.push(pets.advance(menagerie, stage, [{ kind: "ticked", ticks: Math.max(1, Math.ceil(state.lasts * 64) - (stage.tick - actor.stateSince)) }]));
      assert.ok(visited.size <= species.states.length * period);
    }
    assert.deepEqual([...actual].sort(), expected, `${species.id} default-state keyboard reachability`);
    reachable += actual.size;
  }
  console.log(`[DEBUG] SQLite recursive hand-state oracle + actual default-state keyboard sequences PASS: ${reachable} reachable states across ${menagerie.species.length} species, with timed returns and preserved click counters`);
} finally {
  database.close();
}
console.log("[DEBUG] direct pure keyboard gesture regression PASS; managed Rust/React/browser gates remain pending");
