#!/usr/bin/env bun
/** 🖼️ Mounts the `view-linework` inference module in the inferences mount tree of the artifact root, right after `plan_linework` (idempotent). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, em, RS, schema, child, rel } from "./r3-f1-paths.ts";
import { relRoot } from "./r3-f1-gen-leaf.ts";

const root = join(artifact, RS);
const source = readFileSync(root, "utf8");
if (source.includes("pub mod view_linework")) {
  console.log("already mounted");
} else {
  const inferences = child(schema, "inferences");
  const module = join(inferences, em(0x1f5bc) + "view-linework", RS);
  const block = `                        #[path = "."]
                        pub mod view_linework {
                            #[path = "${relRoot(module)}"]
                            mod component;
                            pub use component::*;
                        }
`;
  const anchor = /( {24}pub mod plan_linework \{\n(?: {28}.*\n)+ {24}\}\n)/;
  if (!anchor.test(source)) throw new Error("plan_linework mount not found");
  writeFileSync(root, source.replace(anchor, `$1${block}`));
  console.log("mounted");
}
void rel;
