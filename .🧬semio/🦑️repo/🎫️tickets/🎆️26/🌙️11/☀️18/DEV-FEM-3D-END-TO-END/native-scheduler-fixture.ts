import { realpathSync } from "node:fs";
import { resolve } from "node:path";

/** 🧰 Supplies the scheduler built by the component driver to its native broker integration test. */
const scheduler = realpathSync(resolve(process.env.FEM_SCHEDULER_BINARY!));
Object.assign(globalThis, { __semioScheduler: Promise.resolve(new Map([["semio-build-scheduler", scheduler]])) });
