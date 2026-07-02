#!/usr/bin/env bun
/** 🃏 `@semio-tech/graph-dsl` — shared Jack query language over queryable graphs. */
import { BundleScript, runCargo, runBundleScriptMain } from "../../../repo/lib/js/index.ts";
import { join } from "node:path";

class GraphDslScript extends BundleScript {
  async test(): Promise<void> {
    await runCargo(["test", "-p", "mathematical_graph_dsl"], { cwd: join(this.root, "rs") });
  }
}

const router = new GraphDslScript();
runBundleScriptMain(router, import.meta.url);
