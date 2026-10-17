#!/usr/bin/env bun
/** 📦️ Block5d owning package, canonical Source consumers and snapshot proof. */
import {runArtifactRustPackageMain} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { runCmd } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

/** 🛂️ Verify the exact canonical provider through every published owning Source consumer. */
class OwnedVerifyScript extends BundleScript {
  run(args: string[]): void {
    if (args.length !== 1 || args[0] !== "snapshot-sqlite-source") throw new Error("verify snapshot-sqlite-source");
    const roots: string[] = [];
    const visit = (dir: string): void => {
      for (const item of readdirSync(dir, { withFileTypes: true })) {
        const path = resolve(dir, item.name);
        if (item.isDirectory() && (item.name !== "🧪️tests" || path.includes("📸️snapshot"))) visit(path);
        else if (item.name.endsWith(".ts")) roots.push(path);
      }
    };
    visit(resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any"));
    visit(resolve(this.root, "../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"));
    visit(resolve(this.root, "../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"));
    roots.push(resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts"),resolve(this.root,"../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts"));
    runCmd("bun", [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...roots], { cwd: this.repoRoot });
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-block-5d", {packages:["semio-s-artifact-block-5d-diff","semio-s-artifact-block-5d-editor","semio-s-artifact-block-5d-examples","semio-s-artifact-block-5d-inferences","semio-s-artifact-block-5d-io","semio-s-artifact-block-5d-model","semio-s-artifact-block-5d-mut-cameras","semio-s-artifact-block-5d-mut-grip-kinds","semio-s-artifact-block-5d-mut-grips","semio-s-artifact-block-5d-mut-meta","semio-s-artifact-block-5d-mut-part-kind","semio-s-artifact-block-5d-mut-representations","semio-s-artifact-block-5d-mutations","semio-s-artifact-block-5d-schema","semio-s-artifact-block-5d-viewer","semio-s-artifact-block-5d"],commands:{verify:OwnedVerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts","../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts"]});
