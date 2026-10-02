#!/usr/bin/env bun
/** 🌱️ Seeds a development proctor with learners who submitted runs, so the leaderboards have rows to look at.
 *
 * `bun <this file> <proctor origin> [learners] [tenant]` — every learner registers under a pseudonym and submits one to
 * three runs of random quizzes with random valid answers, through the quiz client's own envelopes. Runs are stamped by
 * the proctor's clock, so every seeded run is of today, this week and this month.
 *
 * @see ../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts — the client driven here
 */
import type { Answer, SheetTask } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🟦️.ts";
import { ProctorClient, newId } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts";
import type { HttpRequest, HttpResponse, HttpTransport } from "../../../../../../../🧰️framework/🛍️products/🖥️server/🟦️.ts";

const [origin = "http://127.0.0.1:8793", count = "12", tenant = "architecture"] = process.argv.slice(2);
const NAMES = ["Mira", "Ben", "Chiara", "Deniz", "Elif", "Finn", "Greta", "Hugo", "Ines", "Jonas", "Klara", "Luca", "Malik", "Nora", "Oskar", "Pia"];

const transport: HttpTransport = {
  async send(request: HttpRequest): Promise<HttpResponse> {
    const response = await fetch(`${origin}${request.path}`, { method: request.method, headers: request.headers, body: request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body) });
    const text = await response.text();
    return { status: response.status, text: async () => text, bytes: async () => new TextEncoder().encode(text) };
  },
};

function shuffled<T>(items: readonly T[]): T[] {
  const order = [...items];
  for (let index = order.length - 1; index > 0; index -= 1) {
    const other = Math.floor(Math.random() * (index + 1));
    [order[index], order[other]] = [order[other]!, order[index]!];
  }
  return order;
}

function answerOf(task: SheetTask): Answer {
  const items = task.items.map((item) => item.id);
  if (task.kind === "sorting") return { kind: "sorting", order: shuffled(items) };
  if (task.kind === "classification") return { kind: "classification", assignments: Object.fromEntries(items.map((item) => [item, task.categories[Math.floor(Math.random() * task.categories.length)]!.id])) };
  return { kind: "matching", assignments: Object.fromEntries(task.dimensions.map((dimension) => [dimension.id, Object.fromEntries(shuffled(dimension.cards.map((_, card) => card)).map((card, index) => [items[index]!, card]))])) };
}

const client = new ProctorClient(() => transport, tenant);
const catalog = await client.catalog(undefined);
for (let index = 0; index < Number(count); index += 1) {
  const learner = newId();
  const handle = `${NAMES[index % NAMES.length]} ${newId().slice(0, 3)}`;
  await client.command({ type: "identify-learner", id: newId(), learner, identity: { kind: "pseudonym", handle } });
  const played: string[] = [];
  for (const quiz of shuffled(catalog.quizzes).slice(0, 1 + (index % 3))) {
    const run = newId();
    await client.command({ type: "start-run", id: newId(), learner, run, quiz: quiz.id });
    const view = await client.run(run, learner);
    for (const task of view.sheet.tasks) await client.command({ type: "record-answer", id: newId(), learner, run, task: task.id, answer: answerOf(task) });
    await client.command({ type: "submit-run", id: newId(), learner, run });
    played.push(quiz.id);
  }
  console.log(`${handle}: ${played.join(", ")}`);
}
const board = await client.leaderboard({ period: "daily" }, undefined);
console.log(`today: ${board.learners} learners, ${board.submissions} submissions, window ${new Date(board.window?.from ?? 0).toISOString()} – ${new Date(board.window?.until ?? 0).toISOString()}`);
