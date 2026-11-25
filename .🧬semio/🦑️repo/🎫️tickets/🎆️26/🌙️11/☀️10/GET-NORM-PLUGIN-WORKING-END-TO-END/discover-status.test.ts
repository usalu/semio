import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import fixture from "./discover-status-fixture.json";

const generated = join(import.meta.dirname, "🗑️generated");
const script = readFileSync(join(import.meta.dirname, "discover-ceilings.sh"), "utf8");
const failure = "could not compile `mock-crate`";
const mock = 'index=1; repo() { local code=${codes[index++]}; if [ "$code" -ne 0 ]; then printf "%s\\n" "$MOCK_COMPILE_FAILURE"; fi; return "$code"; }; codes=(0 "$@")';

for (const row of fixture.cases) test(`discovery reports ${row.statuses.join(", ")} and exits ${row.exit}`, () => {
  mkdirSync(generated, { recursive: true });
  const scratch = mkdtempSync(join(generated, "discover-status-"));
  const output = join(scratch, "🗑️generated");
  mkdirSync(output);
  writeFileSync(join(scratch, "source-gate.sh"), "#!/usr/bin/env bash\nexit 0\n", { mode: 0o755 });
  try {
    const result = spawnSync("bash", ["-c", mock + "\n" + script, join(scratch, "discover-ceilings.sh"), ...row.statuses.map(String)], {
      encoding: "utf8", timeout: 5000, env: { ...process.env, MOCK_COMPILE_FAILURE: failure },
    });
    expect(result.status, result.stderr).toBe(row.exit);
    const lines = readFileSync(join(output, "discover-summary.txt"), "utf8").trim().split("\n");
    expect(lines.map(line => line.match(/ (norm[\w-]*) exit (\d+) /u)?.slice(1) ?? line))
      .toEqual([...fixture.components.map((component, index) => [component, String(row.statuses[index])]), "done"]);
    for (const [index, component] of fixture.components.entries()) {
      expect(lines[index]?.includes(failure), component).toBe(row.statuses[index] !== 0);
    }
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});
