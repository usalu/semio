import fs from "node:fs";
import { createRequire } from "node:module";
const fixture = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const require = createRequire(process.cwd() + "/package.json");
const pLimit = require("p-limit");
const delay = seconds => new Promise(resolve => setTimeout(resolve, seconds * 1000));
const results = {};
for (const workers of fixture.workers) {
  const limit = pLimit(workers);
  const starts = [], counts = {}, completed = new Set();
  function submit(id) {
    return limit(async () => {
      starts.push(id);
      const index = counts[id] || 0;
      counts[id] = index + 1;
      await delay(fixture.attempt_seconds);
      const state = fixture.outcomes[id][index];
      if (state === "completed") completed.add(id);
      return state;
    }).then(async state => {
      if (state === "completed") return;
      if (state === "retry") await delay(fixture.retry_seconds);
      else while (!fixture.release_after.every(id => completed.has(id))) await delay(fixture.poll_seconds);
      return submit(id);
    });
  }
  await Promise.all(fixture.initial.map(submit));
  results[workers] = { initial: starts.slice(0, fixture.initial.length), counts };
}
console.log(JSON.stringify(results));
