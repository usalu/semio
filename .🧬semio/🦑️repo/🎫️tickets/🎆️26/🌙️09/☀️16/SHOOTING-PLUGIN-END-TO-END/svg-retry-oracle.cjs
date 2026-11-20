/** ⏰️ Checks the neutral asset attempt timeline against RxJS virtual scheduling. */
const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const { readFileSync } = require("node:fs");
const { join } = require("node:path");
const { concat, takeUntil, timer } = require("rxjs");
const { TestScheduler } = require("rxjs/testing");

const fixture = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖼️icon-render/🖼️svg-pane/🧪️retry.json", "utf8"));
const scheduler = new TestScheduler(assert.deepEqual);
const observe = (cancelAt) => {
  const attempts = [];
  scheduler.run(() => {
    let work = concat(...[0, ...fixture.retryDelayMs].map(delay => timer(delay)));
    if (cancelAt !== undefined) work = work.pipe(takeUntil(timer(cancelAt)));
    const start = scheduler.now();
    work.subscribe(() => attempts.push(scheduler.now() - start));
  });
  return attempts;
};
assert.deepEqual(observe(), fixture.attemptOffsetsMs);
assert.deepEqual(observe(fixture.cancelAtMs), fixture.cancelledAttemptOffsetsMs);
const executable = process.platform === "win32" ? "svg-retry-probe.exe" : "svg-retry-probe";
const native = execFileSync(join(__dirname, "🗑️generated", executable), { encoding: "utf8" });
assert.deepEqual(native.trim().split(/\r?\n/).map(Number), observe());
