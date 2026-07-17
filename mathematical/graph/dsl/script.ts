#!/usr/bin/env bun
/** 🃏 `@semio-tech/graph-dsl` — shared Jack query language over queryable graphs. */
import { BundleScript, ScriptRouter, runCargoTestBudgeted, runBundleScriptMain } from "../../../repo/lib/js/index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(): void {
    runCargoTestBudgeted(["mathematical_graph_dsl"], join(this.root, "rs"));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url);
