/** 📏️ Measures the largest legitimate `POST /commands` and `POST /queries` body the architecture catalog can produce, as
 * the TypeScript client encodes it (payload bytes as a JSON number array), so the proctor's body limit is sized from
 * the real commands. Run from the repo root: `bun <this file>`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

type Task = { kind: string; id: string; items: { id: string }[]; draw?: number; categories?: { id: string }[]; dimensions?: { id: string; cards?: unknown[] }[] };

const catalogPath = resolve("🎓️teaching/🏛️architecture/❓️quiz/🔣️.json");
const catalog = JSON.parse(readFileSync(catalogPath, "utf8")) as { id: string; quizzes: string[] };
const hex = "f".repeat(32);
const longest = (ids: string[], count: number): string[] => [...ids].sort((left, right) => right.length - left.length).slice(0, count);

function envelopeBytes(kind: string, target: { kind: string; id: string }, command: unknown): number {
  const payload = Array.from(new TextEncoder().encode(JSON.stringify(command)));
  const envelope = {
    commandId: hex,
    kind: `quiz.${kind}`,
    version: 1,
    target: { tenant: catalog.id, ...target },
    scope: catalog.id,
    principal: { kind: "user", id: hex },
    session: null,
    device: null,
    payload,
    causalFrontier: null,
    clientHlc: { millis: 1_900_000_000_000, counter: 4_294_967_295 },
    expectedRevision: null,
    idempotencyKey: hex,
    capabilityProof: null,
    trace: { trace_id: hex, span_id: hex },
  };
  return new TextEncoder().encode(JSON.stringify(envelope)).length;
}

const rows: [string, number][] = [];
const handle = "😀".repeat(64);
rows.push(["identify-learner (64 four-byte characters)", envelopeBytes("identify-learner", { kind: "quiz-roster", id: "roster" }, { type: "identify-learner", id: hex, learner: hex, identity: { kind: "pseudonym", handle } })]);
rows.push(["start-run", envelopeBytes("start-run", { kind: "quiz-learner", id: hex }, { type: "start-run", id: hex, learner: hex, run: hex, quiz: "x".repeat(64) })]);
rows.push(["submit-run", envelopeBytes("submit-run", { kind: "quiz-learner", id: hex }, { type: "submit-run", id: hex, learner: hex, run: hex })]);

for (const relative of catalog.quizzes) {
  const quiz = JSON.parse(readFileSync(resolve(dirname(catalogPath), relative), "utf8")) as { id: string; tasks: Task[] };
  for (const task of quiz.tasks) {
    const drawn = longest(task.items.map((item) => item.id), task.draw ?? task.items.length);
    const whole = task.items.map((item) => item.id);
    for (const [label, items] of [["drawn", drawn], ["every item", whole]] as const) {
      const answer =
        task.kind === "sorting"
          ? { kind: "sorting", order: items }
          : task.kind === "classification"
            ? { kind: "classification", assignments: Object.fromEntries(items.map((item) => [item, longest(task.categories!.map((category) => category.id), 1)[0]])) }
            : { kind: "matching", assignments: Object.fromEntries(task.dimensions!.map((dimension) => [dimension.id, Object.fromEntries(items.map((item, index) => [item, 1000 + index]))])) };
      rows.push([`record-answer ${quiz.id}/${task.id} (${label}, ${items.length} items)`, envelopeBytes("record-answer", { kind: "quiz-learner", id: hex }, { type: "record-answer", id: hex, learner: hex, run: hex, task: task.id, answer })]);
    }
  }
}

const query = { queryId: hex, kind: "quiz.leaderboard", version: 1, scope: catalog.id, principal: { kind: "user", id: hex }, arguments: Array.from(new TextEncoder().encode(JSON.stringify({ type: "learner", learner: hex }))), consistency: { kind: "authority" }, cursor: null };
rows.push(["query (learner)", new TextEncoder().encode(JSON.stringify(query)).length]);

rows.sort((left, right) => right[1] - left[1]);
for (const [label, bytes] of rows) console.log(`${String(bytes).padStart(6)}  ${label}`);
console.log(`largest: ${rows[0][1]} bytes`);
