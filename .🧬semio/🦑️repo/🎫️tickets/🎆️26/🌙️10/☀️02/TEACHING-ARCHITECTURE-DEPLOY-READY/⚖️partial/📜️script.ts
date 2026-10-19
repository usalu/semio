import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";

const root = process.cwd();
const product = resolve(root, "🧰️framework/🛍️products/📚️course");
const modules = resolve(product, "🎯️targets/⚛️react/🔨️modules");
const load = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const imported = (path: string) => import(pathToFileURL(path).href);
const { scoreTask, scoreRun } = await imported(resolve(product, "🔨️modules/📏️scoring/🟦️.ts"));
const { answerRejection, answerComplete } = await imported(resolve(product, "🔨️modules/✅️validation/🟦️.ts"));
const { CourseSession } = await imported(resolve(modules, "🧭️session/🟦️.ts"));
const { Deputy } = await imported(resolve(modules, "🫡️deputy/🟦️.ts"));
const { ProctorClient, ProctorUnavailable } = await imported(resolve(modules, "🛂️proctor/🟦️.ts"));
const { localStore, memoryStorageOrigin } = await imported(resolve(modules, "💾️persistence/🟦️.ts"));
const { ARCHITECTURE_COURSE_MATERIAL: material } = await imported(resolve(root, "🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts"));
const schema = load(resolve(product, "🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const valid = (name: string, value: unknown) => assert.equal(ajv.validate(`${schema.$id}#/$defs/${name}`, value), true, `${name}: ${JSON.stringify(ajv.errors)}`);
const close = (actual: number, expected: number, label: string) => assert.ok(Math.abs(actual - expected) < 1e-12, `${label}: ${actual} != ${expected}`);
const db = new Database(":memory:");
db.exec("CREATE TABLE input(task TEXT, sheet TEXT, answer TEXT)");
const insert = db.query("INSERT INTO input VALUES (?, ?, ?)");
const PAIRS = `WITH
  dimensions AS (
    SELECT json_extract(d.value, '$.id') id, d.value document FROM input, json_each(sheet, '$.dimensions') d
    UNION ALL SELECT '', NULL FROM input WHERE json_extract(task, '$.kind') = 'sorting'
  ),
  raw AS (
    SELECT dimensions.id dimension, presented.key ordinal,
      json_extract(actual.value, '$.value') sorting_truth,
      json_extract(actual.value, '$.values.' || json_quote(dimensions.id)) matching_truth,
      coalesce((SELECT key FROM json_each(input.answer, '$.order') WHERE value = json_extract(presented.value, '$.id')), presented.key) position,
      coalesce(json_extract(dimensions.document, '$.quantity.scale'), json_extract(task, '$.quantity.scale')) scale,
      json_extract(task, '$.kind') kind,
      CASE WHEN json_extract(task, '$.kind') = 'sorting' THEN json_type(sheet, '$.keys') IS NULL ELSE json_type(dimensions.document, '$.cards') IS NULL END hidden,
      input.answer IS NOT NULL answered,
      json_extract(input.answer, '$.guesses.' || CASE WHEN dimensions.id = '' THEN '' ELSE json_quote(dimensions.id) || '.' END || json_quote(json_extract(presented.value, '$.id'))) guess,
      json_extract(dimensions.document, '$.cards[' || json_extract(input.answer, '$.assignments.' || json_quote(dimensions.id) || '.' || json_quote(json_extract(presented.value, '$.id'))) || ']') card
    FROM input, dimensions, json_each(sheet, '$.items') presented
    JOIN json_each(task, '$.items') actual ON json_extract(actual.value, '$.id') = json_extract(presented.value, '$.id')
  ), values_ AS (
    SELECT *, CASE WHEN kind = 'sorting' THEN sorting_truth ELSE matching_truth END truth,
      CASE WHEN hidden THEN guess ELSE card END assigned FROM raw
  ), bounded AS (
    SELECT *, min(truth) OVER (PARTITION BY dimension) low, max(truth) OVER (PARTITION BY dimension) high FROM values_
  ), evaluated AS (
    SELECT *, CASE WHEN scale = 'logarithmic' THEN log10(truth) ELSE truth END value,
      CASE WHEN kind = 'sorting' AND NOT hidden THEN NOT answered
        WHEN assigned IS NULL THEN 1
        WHEN NOT hidden THEN 0
        WHEN scale = 'logarithmic' THEN max(1.0 * assigned / truth, 1.0 * truth / assigned) > (CASE WHEN high > low THEN min(1000.0, sqrt(1.0 * high / low)) ELSE 1000.0 END) * (1 + 1e-9)
        ELSE high > low AND abs(assigned - truth) > (high - low) / 2.0 * (1 + 1e-9) END missed
    FROM bounded
  ), pairs AS (
    SELECT a.dimension, abs(a.value - b.value) weight,
      CASE WHEN a.missed OR b.missed THEN 0.0
        WHEN a.kind = 'sorting' THEN CASE WHEN a.truth <= b.truth THEN 1.0 ELSE 0.0 END
        WHEN (a.truth - b.truth) * (a.assigned - b.assigned) > 0 THEN 1.0
        WHEN a.assigned = b.assigned THEN 0.5 ELSE 0.0 END credit
    FROM evaluated a JOIN evaluated b ON a.dimension = b.dimension AND a.position < b.position
  ), scores AS (
    SELECT id dimension, CASE WHEN coalesce((SELECT sum(weight) FROM pairs WHERE dimension = dimensions.id), 0) > 0
      THEN (SELECT 1.0 * sum(weight * credit) / sum(weight) FROM pairs WHERE dimension = dimensions.id)
      ELSE CASE WHEN (SELECT max(missed) FROM evaluated WHERE dimension = dimensions.id) THEN 0.0 ELSE 1.0 END END score FROM dimensions
  ) SELECT dimension, score FROM scores`;
const PROFILES = `WITH
  categories AS (SELECT json_extract(value, '$.id') id, value document FROM input, json_each(task, '$.categories')),
  axes AS (SELECT value document FROM input, json_each(task, '$.axes')),
  deltas AS (
    SELECT a.id a, b.id b,
      1.0 * (json_extract(a.document, '$.profile.' || json_quote(json_extract(axes.document, '$.id'))) - json_extract(b.document, '$.profile.' || json_quote(json_extract(axes.document, '$.id')))) /
      (json_extract(axes.document, '$.max') - json_extract(axes.document, '$.min')) delta
    FROM categories a, categories b, axes
  ), distances AS (SELECT a, b, sqrt(sum(delta * delta)) distance FROM deltas GROUP BY a, b HAVING count(delta) = (SELECT count(*) FROM axes)),
  presented AS (
    SELECT json_extract(actual.value, '$.category') truth,
      json_extract(answer, '$.assignments.' || json_quote(json_extract(shown.value, '$.id'))) assigned
    FROM input, json_each(sheet, '$.items') shown JOIN json_each(task, '$.items') actual ON json_extract(actual.value, '$.id') = json_extract(shown.value, '$.id')
  ) SELECT avg(CASE WHEN assigned IS NULL THEN 0.0 WHEN assigned = truth THEN 1.0
      ELSE coalesce(max(0.0, 1 - (SELECT distance FROM distances WHERE a = assigned AND b = truth) / (SELECT max(distance) FROM distances)), 0.0) END) score FROM presented`;
function oracle(task: any, sheet: any, answer?: any) {
  db.exec("DELETE FROM input");
  insert.run(JSON.stringify(task), JSON.stringify(sheet), answer === undefined ? null : JSON.stringify(answer));
  const scores = db.query(task.kind === "classification" ? PROFILES : PAIRS).all() as { dimension?: string; score: number }[];
  return { score: scores.reduce((sum, entry) => sum + entry.score, 0) / scores.length, dimensions: scores };
}
let neutral = 0;
for (const folder of ["📏️sorting-concordance", "🔀️matching-concordance", "🕸️profile-similarity"]) {
  const fixture = load(resolve(product, "🧫️fixtures", folder, "🔣️.json"));
  for (const vector of [...fixture.vectors, ...fixture.guessed ?? [], ...fixture.timed ?? []]) {
    const task = fixture.tasks.find((candidate: any) => candidate.id === vector.task);
    const expected = vector.expected.score;
    const actual = oracle(task, vector.sheetTask, vector.answer);
    if (Math.abs(actual.score - expected) >= 1e-12) console.log("[DEBUG] oracle mismatch input", vector, db.query(PAIRS.replace("SELECT dimension, score FROM scores", "SELECT * FROM evaluated")).all());
    close(actual.score, expected, `${folder}/${vector.id} SQLite vs neutral expected`);
    close(scoreTask(task, vector.sheetTask, vector.answer).score, expected, `${folder}/${vector.id} owned vs neutral expected`);
    neutral++;
  }
}
console.log(`[DEBUG] SQLite relational scoring oracle calibrated on ${neutral} existing neutral vectors PASS`);

function answerOf(task: any, presented: any, mode: string) {
  const items = presented.items.map((item: any) => task.items.find((known: any) => known.id === item.id));
  const partial = mode === "partial";
  const count = partial ? Math.max(1, Math.floor(items.length / 2)) : items.length;
  const swap = (values: number[]) => {
    const sorted = values.map((value, index) => ({ value, index })).sort((a, b) => a.value - b.value);
    if (mode === "extreme") return [sorted[0]!.index, sorted.at(-1)!.index];
    const candidates = sorted.slice(1).flatMap((entry, index) => entry.value > sorted[index]!.value ? [{ first: sorted[index]!.index, second: entry.index, distance: Math.abs(Math.log10(entry.value) - Math.log10(sorted[index]!.value)) }] : []);
    const nearest = candidates.reduce((one, other) => one.distance <= other.distance ? one : other);
    return [nearest.first, nearest.second];
  };
  if (task.kind === "classification") {
    const assignments = Object.fromEntries(items.slice(0, count).map((item: any) => [item.id, item.category]));
    if (!partial) {
      const first = items[0];
      const correct = task.categories.find((category: any) => category.id === first.category);
      const choices = task.categories.filter((category: any) => category.id !== correct.id).map((category: any) => ({ category, distance: task.axes?.length && correct.profile && category.profile ? task.axes.reduce((sum: number, axis: any) => sum + ((correct.profile[axis.id] - category.profile[axis.id]) / (axis.max - axis.min)) ** 2, 0) : 1 }));
      choices.sort((a: any, b: any) => a.distance - b.distance);
      assignments[first.id] = (mode === "extreme" ? choices.at(-1) : choices[0]).category.id;
    }
    return { kind: task.kind, assignments };
  }
  if (task.kind === "sorting") {
    let order = [...items].sort((a: any, b: any) => a.value - b.value).map((item: any) => item.id);
    const values = items.map((item: any) => item.value);
    const given = Object.fromEntries(items.map((item: any) => [item.id, item.value]));
    if (!partial) {
      const [one, other] = swap(values);
      [given[items[one].id], given[items[other].id]] = [given[items[other].id], given[items[one].id]];
      order.sort((a: string, b: string) => given[a] - given[b]);
    }
    const guesses = Object.fromEntries(order.slice(0, count).map((id: string) => [id, given[id]]));
    return { kind: task.kind, order, ...(presented.keys === undefined ? { guesses } : {}) };
  }
  const hidden = presented.dimensions[0].cards === undefined;
  const valuesByDimension = Object.fromEntries(presented.dimensions.map((dimension: any, index: number) => {
    const values = items.map((item: any) => item.values[dimension.id]);
    const given = [...values];
    if (!partial && index === 0) {
      const [one, other] = swap(values);
      [given[one], given[other]] = [given[other], given[one]];
    }
    const used = new Set<number>();
    return [dimension.id, Object.fromEntries(items.slice(0, count).map((item: any, position: number) => {
      if (hidden) return [item.id, given[position]];
      const card = dimension.cards.findIndex((value: number, candidate: number) => value === given[position] && !used.has(candidate));
      assert.ok(card >= 0);
      used.add(card);
      return [item.id, card];
    }))];
  }));
  if (partial) for (const dimension of presented.dimensions.slice(1)) delete valuesByDimension[dimension.id];
  return { kind: task.kind, [hidden ? "guesses" : "assignments"]: valuesByDimension };
}

const par = load(resolve(product, "🧫️fixtures/⛰️challenge-rules/🔣️.json")).rules.expected.table;
const modes = ["partial", "near", "extreme", "blank"];
let runsChecked = 0;
let tasksChecked = 0;
let dimensionsChecked = 0;
let incompleteAnswers = 0;
let omittedTasks = 0;
let nonzeroPartialCredits = 0;
let preservedDimensions = 0;
let profileCredits = 0;
for (const quiz of material.quizzes) {
  for (const challenge of ["easy", "medium", "hard", "expert"]) {
    for (const mode of modes) {
      const proctor = new ProctorClient(() => ({ send: async () => { throw new ProctorUnavailable("offline scoring probe"); } }), material.course.id);
      proctor.silent();
      const origin = memoryStorageOrigin();
      const area = origin.tab();
      const session = new CourseSession({ proctor, store: localStore(area, material.course.id), deputy: new Deputy(material), timing: { minMs: 10000, maxMs: 10000 }, now: () => 1800000000000 });
      session.start();
      try {
        const signal = new AbortController().signal;
        assert.equal(await session.identify({ kind: "anonymous" }, signal), undefined);
        assert.equal(await session.startRun(quiz.id, challenge, signal), undefined);
        const opened = session.getSnapshot().state;
        assert.equal(opened.step.screen, "run");
        const run = opened.step.run;
        const sheet = opened.runs[run].sheet;
        valid("Sheet", sheet);
        const answers: Record<string, any> = {};
        let openings = 0;
        for (const [index, presented] of sheet.tasks.entries()) {
          const task = quiz.tasks.find((known: any) => known.id === presented.id);
          if (mode !== "blank" && !(mode === "partial" && index % 2 === 1)) {
            const answer = answerOf(task, presented, mode);
            assert.equal(answerRejection(presented, answer), undefined);
            valid("Answer", answer);
            if (!answerComplete(presented, answer)) incompleteAnswers++;
            if (challenge === "expert") {
              assert.equal(await session.openTask(run, task.id, signal), undefined);
              openings++;
            }
            session.answer(run, task.id, answer);
            assert.deepEqual(session.getSnapshot().state.runs[run].answers[task.id], answer);
            answers[task.id] = answer;
          } else omittedTasks++;
        }
        assert.equal(session.outbox.queued().length, 2 + openings + Object.keys(answers).length);
        const phases: any[] = [];
        assert.equal(await session.submit(run, signal, (phase: any) => phases.push(phase)), undefined);
        assert.deepEqual(phases, [{ phase: "submitting" }, { phase: "results" }]);
        const view = session.getSnapshot().state.runs[run];
        assert.equal(view.status, "submitted");
        assert.deepEqual(session.getSnapshot().state.step, { screen: "results", run });
        assert.deepEqual(view.answers, answers);
        valid("RunView", view);
        valid("RunResult", view.result);
        valid("LearnerView", session.getSnapshot().state.learnerView);
        assert.deepEqual(view.result, scoreRun(quiz, sheet, answers));
        const expectedScores: number[] = [];
        for (const presented of sheet.tasks) {
          const task = quiz.tasks.find((known: any) => known.id === presented.id);
          const scored = view.result.tasks.find((entry: any) => entry.task === task.id);
          const expected = oracle(task, presented, answers[task.id]);
          close(scored.score, expected.score, `${quiz.id}/${challenge}/${mode}/${task.id}`);
          assert.ok(scored.score >= 0 && scored.score <= 1);
          if (scored.score > 0 && scored.score < 1) nonzeroPartialCredits++;
          if (answers[task.id] === undefined) assert.equal(scored.score, 0);
          if (scored.kind === "matching") for (const dimension of scored.dimensions) {
            close(dimension.score, expected.dimensions.find((entry: any) => entry.dimension === dimension.dimension)!.score, `${task.id}/${dimension.dimension}`);
            dimensionsChecked++;
            if (mode !== "partial" && mode !== "blank" && dimension.dimension !== presented.dimensions[0].id) {
              assert.equal(dimension.score, 1);
              preservedDimensions++;
            }
          }
          if (scored.kind === "classification" && task.axes?.length && mode !== "partial" && mode !== "blank") {
            assert.ok(scored.items.some((item: any) => item.assigned !== item.correct));
            profileCredits += scored.items.filter((item: any) => item.credit > 0 && item.credit < 1).length;
          }
          expectedScores.push(expected.score);
          tasksChecked++;
        }
        const expected = expectedScores.reduce((sum, score) => sum + score, 0) / sheet.tasks.length;
        close(view.result.score, expected, `${quiz.id}/${challenge}/${mode} mean`);
        close(view.result.points, expected * par[challenge].par, `${quiz.id}/${challenge}/${mode} points`);
        assert.ok(view.result.score < 1);
        if (mode === "blank") assert.equal(view.result.score, 0);
        const queue = session.outbox.queued();
        assert.equal(queue.length, 3 + openings + Object.keys(answers).length);
        assert.equal(queue[0].type, "identify-learner");
        assert.equal(queue[1].type, "start-run");
        assert.equal(queue.at(-1).type, "submit-run");
        for (const command of queue) valid("Command", command);
        session.stop();
        const restored = new CourseSession({ proctor, store: localStore(origin.tab(), material.course.id), deputy: new Deputy(material), timing: { minMs: 10000, maxMs: 10000 } });
        assert.deepEqual(restored.getSnapshot().state.runs[run].result, view.result);
        assert.deepEqual(restored.getSnapshot().state.runs[run].answers, answers);
        assert.deepEqual(restored.outbox.queued(), queue);
        console.log(`[DEBUG] ${quiz.id}/${challenge}/${mode} PASS score=${view.result.score} points=${view.result.points} tasks=${sheet.tasks.length} queued=${queue.length} seed=${sheet.seed}`);
        runsChecked++;
      } finally { session.stop(); }
    }
  }
}
assert.ok(incompleteAnswers > 0 && omittedTasks > 0 && nonzeroPartialCredits > 0 && preservedDimensions > 0 && profileCredits > 0);
console.log("[DEBUG] authored partial/nonperfect session matrix PASS", { neutral, runsChecked, tasksChecked, dimensionsChecked, incompleteAnswers, omittedTasks, nonzeroPartialCredits, preservedDimensions, profileCredits, quizzes: material.quizzes.length });
console.log("[DEBUG] actual provisional deputy results, local-first answers, persisted outbox and restored records verified; Rust, authoritative delivery, managed React and browser gates remain pending");
db.close();
