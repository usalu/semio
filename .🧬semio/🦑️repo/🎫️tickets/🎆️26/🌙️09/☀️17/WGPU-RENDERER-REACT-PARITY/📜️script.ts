import { resolve, join, relative, isAbsolute, sep } from "node:path";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { runSceneShadingOracle } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️world3d-scene-shading/📜️script.ts";

type FrameTurnFixture = {
  readonly input: { readonly inputSequence: number; readonly generation: number };
  readonly turnOwners: readonly ["frame"];
  readonly requests: readonly [{ readonly owner: "frame"; readonly remainingTurns: number }];
  readonly runtimeTurns: readonly { readonly phase: "uploads" | "finish" | "terminal"; readonly requestFrame: boolean; readonly continueFrame: boolean }[];
  readonly nonRunnableTurns: readonly { readonly owner: "future-deadline" | "hub-status-external"; readonly requestFrame: boolean; readonly continueFrame: false }[];
  readonly textIngressTurns: readonly { readonly phase: "start" | "partial" | "commit" | "cancel"; readonly committed: boolean; readonly retiring: boolean; readonly continueFrame: boolean }[];
  readonly expected: {
    readonly ingressTrace: readonly string[];
    readonly callbackOwners: readonly string[];
    readonly frameSequences: readonly number[];
    readonly runtimePhases: readonly string[];
    readonly nonRunnableCallbacks: readonly string[];
    readonly runnableTextPhases: readonly string[];
    readonly closeOwners: readonly string[];
    readonly exhaustion: "fault";
  };
};

function nextFrameSequence(current: number): number {
  if (!Number.isSafeInteger(current) || current < 0 || current >= Number.MAX_SAFE_INTEGER) throw new Error("frame output sequence exhausted");
  return current + 1;
}

async function runFrameSchedulerOracle(fixturePath: string): Promise<void> {
  const canonical = join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧵️frame-turn-scheduling/🔣️.json");
  if (resolve(fixturePath) !== canonical) throw new Error("The fixture must be the canonical shared frame-turn fixture");
  const fixture = JSON.parse(await readFile(canonical, "utf8")) as FrameTurnFixture;
  if (fixture.turnOwners.length !== 1 || fixture.turnOwners[0] !== "frame" || fixture.requests.length !== 1 || fixture.requests[0].owner !== "frame") throw new Error("The bounded oracle accepts only the implemented frame owner");
  const scheduler = createRequire(import.meta.url)("scheduler") as {
    readonly unstable_NormalPriority: number;
    readonly unstable_scheduleCallback: (priority: number, callback: () => void) => unknown;
  };
  const ingressTrace = [`enqueue:${fixture.input.inputSequence}`, `batch-accepted:${fixture.input.inputSequence}`];
  const callbackOwners: string[] = [];
  const frameSequences: number[] = [];
  const closeOwners: string[] = [];
  let remaining = fixture.requests[0].remainingTurns;
  let frameSequence = 0;
  await new Promise<void>((done) => {
    const requestFrameTurn = (): void => {
      scheduler.unstable_scheduleCallback(scheduler.unstable_NormalPriority, () => {
        callbackOwners.push("frame");
        frameSequence = nextFrameSequence(frameSequence);
        frameSequences.push(frameSequence);
        remaining -= 1;
        if (remaining > 0) requestFrameTurn();
        else scheduler.unstable_scheduleCallback(scheduler.unstable_NormalPriority, () => {
          closeOwners.push("frame");
          done();
        });
      });
    };
    requestFrameTurn();
  });
  const runtimePhases: string[] = [];
  const runtimeTurns = [...fixture.runtimeTurns];
  await new Promise<void>((done) => {
    const requestRuntimeTurn = (): void => {
      scheduler.unstable_scheduleCallback(scheduler.unstable_NormalPriority, () => {
        const turn = runtimeTurns.shift();
        if (!turn) throw new Error("runtime frame turn credits exhausted");
        runtimePhases.push(turn.phase);
        if (turn.continueFrame) requestRuntimeTurn();
        else done();
      });
    };
    requestRuntimeTurn();
  });
  if (runtimeTurns.length) throw new Error("runtime frame oracle terminated before consuming its declared turns");
  const nonRunnableCallbacks: string[] = [];
  await Promise.all(fixture.nonRunnableTurns.map((turn) => new Promise<void>((done) => {
    scheduler.unstable_scheduleCallback(scheduler.unstable_NormalPriority, () => {
      nonRunnableCallbacks.push(turn.owner);
      if (turn.continueFrame) throw new Error(`${turn.owner} illegally requested a private continuation`);
      done();
    });
  })));
  const runnableTextPhases = fixture.textIngressTurns.filter((turn) => turn.committed || turn.retiring).map((turn) => turn.phase);
  for (const turn of fixture.textIngressTurns) {
    if (turn.continueFrame !== (turn.committed || turn.retiring)) throw new Error(`text ingress continuation mismatch: ${turn.phase}`);
  }
  let exhaustion = "accepted";
  try {
    nextFrameSequence(Number.MAX_SAFE_INTEGER);
  } catch {
    exhaustion = "fault";
  }
  const actual = { ingressTrace, callbackOwners, frameSequences, runtimePhases, nonRunnableCallbacks, runnableTextPhases, closeOwners, exhaustion };
  if (JSON.stringify(actual) !== JSON.stringify(fixture.expected)) throw new Error(`frame scheduler oracle mismatch: ${JSON.stringify(actual)}`);
  console.log(`frame-scheduler-oracle: input=${fixture.input.inputSequence} generation=${fixture.input.generation} callbacks=${callbackOwners.length} runtime=${runtimePhases.join("→")} close=${closeOwners.length} exhaustion=${exhaustion}`);
}

const [command, fixture, output, ...rest] = process.argv.slice(2);
if (command === "frame-scheduler-oracle") {
  if (!fixture || output || rest.length) throw new Error("Usage: frame-scheduler-oracle <fixture>");
  await runFrameSchedulerOracle(fixture);
} else {
  if ((command !== "shading-oracle" && command !== "shading-wgpu") || !fixture || !output || rest.length) throw new Error("Usage: shading-oracle|shading-wgpu <fixture> <ticket-generated-output-directory>");
  if (resolve(fixture) !== join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading/🔣️.json")) throw new Error("The fixture must be the canonical shared World3d shading fixture");
  const outputRelative = relative(join(import.meta.dir, "🗑️generated"), resolve(output));
  if (!outputRelative || outputRelative === ".." || outputRelative.startsWith(".." + sep) || isAbsolute(outputRelative)) throw new Error("Outputs must stay under the ticket generated directory");
  await runSceneShadingOracle(process.cwd(), command, resolve(output));
}
