import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:80], s.count(old))
    s = s.replace(old, new)
rep("/** 🔭️ The same live corpus runs", "/** 🌍️ The same live corpus runs")
rep('rounds: row.rounds, timeoutMs: 120000, backend, compare: "output"});', 'rounds: row.rounds, timeoutMs: 120000, backend, compare: "output"}, {profileCapacity: (row as {capacity?: number}).capacity});')
s = s.rstrip("\n") + '''

/** 🔀️ A plan that needs more profiles than the request allows hands over to the mirror (no profile limit) and says so, instead of failing or relaxing the plan. */
test.skipIf(process.platform !== "darwin" || !availability.seatbelt.available)("a plan that needs more profiles than allowed falls back to the mirror backend", async () => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), "ro-fall-"))), said: string[] = [];
  try {
    for (const name of ["m1", "m2", "m3", "m4"]) { mkdirSync(join(root, "many"), {recursive: true}); writeFileSync(join(root, "many", name), name); }
    const request = {version: 1, language: "en", root, cwd: "", allowed: ["many/m1", "many/m3"], ambient: [], names: [], command: ["sh", "-c", "cat many/m1 many/m3"], environment: {}, rounds: 1, timeoutMs: 60000, backend: "auto", compare: "exit"};
    const report = await runReadOracleV1({...request, maxProfiles: 1}, {profileCapacity: 1, progress: (text) => said.push(text)});
    expect(report.backend).toBe("mirror");
    expect(report.strength).toBe("output-equivalence");
    expect(report.clean).toBe(true);
    expect(said.some((text) => text.includes("seatbelt") && text.includes("mirror"))).toBe(true);
    const split = await runReadOracleV1({...request, allowed: ["many/m1", "many/m2", "many/m3"], maxProfiles: 4}, {profileCapacity: 2, progress: (text) => said.push(text)});
    expect(split.backend).toBe("seatbelt");
    expect(split.rounds[0]!.notes).toContain("profiles:1");
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
});

test("a command started with an already aborted signal is stopped at once", async () => {
  const controller = new AbortController();
  controller.abort();
  const done = await captureV1(["sleep", "30"], {cwd: process.cwd(), env: {PATH: process.env.PATH}, timeoutMs: 60000, signal: controller.signal});
  expect(done.exit).toBe(124);
  expect(done.seconds).toBeLessThan(10);
});
'''
open(p, "w", encoding="utf-8").write(s)
