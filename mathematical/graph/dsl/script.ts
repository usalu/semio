#!/usr/bin/env bun
/** 🃏 `@semio-tech/graph-dsl` — shared Jack query language over queryable graphs. */
import { BundleScript, runCargoTestBudgeted, runBundleScriptMain } from "../../../repo/lib/js/index.ts";
import { join } from "node:path";

class GraphDslScript extends BundleScript {
  async test(): Promise<void> {
    runCargoTestBudgeted(["mathematical_graph_dsl"], join(this.root, "rs"));
  }
}

const router = new GraphDslScript();
runBundleScriptMain(router, import.meta.url);
