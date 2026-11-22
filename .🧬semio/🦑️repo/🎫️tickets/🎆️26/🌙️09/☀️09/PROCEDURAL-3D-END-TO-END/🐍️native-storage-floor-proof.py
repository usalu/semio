"""🛑 Tests the native component driver's storage and diagnostic policies."""
from pathlib import Path
import json
import subprocess

out = Path(__file__).parent / "🗑️generated"
base = Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass")
source = (base / "🟦️.ts").read_text()
case = json.loads((base / "🧪️tests/🧫️fixtures/storage-floor.json").read_text())
functions = source[source.index("function boundedLog("):source.index("const exec = promisify")]
proof = out / "native-storage-floor.test.ts"
proof.write_text('import { test, expect } from "bun:test";\nconst fixture = ' + json.dumps(case) + r""";
const chunks: Buffer[] = [];
let failure: "open" | "write" | "other" | undefined;
let closed = 0;
const noSpace = () => Object.assign(new Error("disk"), {code: "ENOSPC"});
const openSync = () => { if (failure === "open") throw noSpace(); return 5; };
const closeSync = () => { closed++; };
const writeSync = (_: number, bytes: Uint8Array | string) => {
  if (failure === "write") throw noSpace();
  if (failure === "other") throw Error("broken file");
  const chunk = Buffer.from(bytes);
  chunks.push(chunk);
  return chunk.length;
};
""" + functions + r"""
test("only confirmed physical headroom below the admitted reserve requests recovery", () => {
  for (const row of fixture.pressure) {
    let rechecked = false;
    const error = nativeBuildStorageFloor(row.available, row.reserve, () => { rechecked = true; return row.confirmed; });
    expect(rechecked).toBe(row.rechecked);
    expect(!!error).toBe(row.stop);
    if (error) { expect(error.code).toBe("ENOSPC"); expect(error.message).toContain("No space left on device"); }
  }
});

test("diagnostic log accepts complete UTF-8 records within its byte cap", () => {
  const log = boundedLog("cargo", fixture.log.limit, () => { throw Error("unexpected storage loss"); });
  log.write(fixture.log.first);
  log.write(fixture.log.oversize);
  log.write(fixture.log.normal);
  log.close();
  expect(Buffer.concat(chunks).toString()).toBe(fixture.log.first + "\n" + fixture.log.normal + "\n");
});

test("log opening and writing ENOSPC do not break compiler diagnostics", () => {
  for (const where of ["open", "write"] as const) {
    const before = closed;
    const paths: string[] = [];
    failure = where;
    const log = boundedLog("cargo", 20, path => paths.push(path));
    log.write("diagnostic");
    log.write("next diagnostic");
    log.close();
    expect(closed - before).toBe(where === "open" ? 0 : 1);
    expect(paths).toEqual(["cargo"]);
  }
  failure = "other";
  const log = boundedLog("cargo", 20, () => { throw Error("unexpected capacity error"); });
  expect(() => log.write("error")).toThrow("broken file");
  log.close();
});
""")
telemetry = (base.parent / "📈️telemetry/🟦️.ts").read_text()
if "function timelineAppender(" in telemetry:
    function = telemetry[telemetry.index("function timelineAppender("):telemetry.index("/** 🔰️ Samples machine")]
else:
    function = ""
with proof.open("a") as tests:
    tests.write(function + r"""
let timelineFailure: "mkdir" | "append" | "other" | undefined;
let recorded: string[] = [];
const dirname = () => "timeline directory";
const mkdirSync = () => { if (timelineFailure === "mkdir") throw noSpace(); };
const appendFileSync = (_: string, line: string) => {
  if (timelineFailure === "append") throw noSpace();
  if (timelineFailure === "other") throw Error("different failure");
  recorded.push(line);
};

test("timeline disk exhaustion stops persistence while accepting later observations", () => {
  const paths: string[] = [];
  timelineFailure = "mkdir";
  const unavailable = timelineAppender("timeline", path => paths.push(path));
  timelineFailure = undefined;
  unavailable("later\n");
  expect(paths).toEqual(["timeline"]);
  expect(recorded).toEqual([]);
  const recorder = timelineAppender("timeline2", path => paths.push(path));
  recorder("first\n");
  timelineFailure = "append";
  recorder("full\n");
  timelineFailure = undefined;
  recorder("later\n");
  expect(paths).toEqual(["timeline", "timeline2"]);
  expect(recorded).toEqual(["first\n"]);
  const other = timelineAppender("timeline3");
  timelineFailure = "other";
  expect(() => other("different\n")).toThrow("different failure");
});
""")
subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", "test", str(proof)], check=True)
