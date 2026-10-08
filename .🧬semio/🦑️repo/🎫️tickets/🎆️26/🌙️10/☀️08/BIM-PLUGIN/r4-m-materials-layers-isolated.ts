#!/usr/bin/env bun
/**
 * 🔬️ Isolated build of the `m-materials-layers` leaves. While sibling agents' editor/io/inference work is in flight the full crate does not
 * compile, and the mutation derives insist that an aggregate and its leaves live side by side in one real (non-symlinked) taxonomy tree.
 * `bun r4-m-materials-layers-isolated.ts` therefore builds a private tree under `🗑️generated/m-materials-layers/priv` (a private cargo
 * workspace, see `priv-setup.ts` there): the real snapshot and diff sources are mounted by absolute path, while the mutation aggregate (cut
 * down to this slice), the kit, the 12 leaf directories and their fixtures are copied next to it. `--collect` copies the blessed `after`
 * and `diff` fixtures of the applied cases back into the real fixture tree.
 */
import { copyFileSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { artifact, child, em, fixtures, mutations, repo, RS } from "./r3-f1-paths.ts";
import { leaves } from "./r3-m-materials-layers-leaves.ts";

const priv = join(import.meta.dir, "🗑️generated", "m-materials-layers", "priv");
const mine = new Set(leaves.map((leaf) => leaf.kind.replaceAll("-", "_")));
const leafDirs = leaves.map((leaf) => em(leaf.emoji) + leaf.kind);
const abs = (path: string) => path.replaceAll("\\", "/");

const copyTree = (from: string, to: string) => {
  mkdirSync(to, { recursive: true });
  for (const name of readdirSync(from)) {
    const source = join(from, name);
    if (statSync(source).isDirectory()) copyTree(source, join(to, name));
    else copyFileSync(source, join(to, name));
  }
};

const privRepo = join(priv, "repo");
const privArtifact = join(privRepo, relative(repo, artifact));
const privMutations = join(privArtifact, relative(artifact, mutations));
const privFixtures = join(privArtifact, relative(artifact, join(fixtures, em(0x1f9ec) + "mutations")));
const realFixtures = join(fixtures, em(0x1f9ec) + "mutations");

if (process.argv.includes("--collect")) {
  let moved = 0;
  for (const leafDir of leafDirs) {
    for (const caseDir of readdirSync(join(privFixtures, leafDir))) {
      for (const [folder, name] of [[em(0x1f4f8) + "snapshot/" + em(0x27a1) + "after", "after"], [em(0x1f53a) + "diff", "diff"]]) {
        const source = join(privFixtures, leafDir, caseDir, folder, "\u{1f523}️.json");
        const target = join(realFixtures, leafDir, caseDir, folder, "\u{1f523}️.json");
        const outcome = JSON.parse(readFileSync(join(privFixtures, leafDir, caseDir, em(0x1f3af) + "outcome", "\u{1f523}️.json"), "utf8"));
        if (outcome.status === "applied" && readFileSync(source, "utf8") !== readFileSync(target, "utf8")) {
          copyFileSync(source, target);
          moved++;
        }
        void name;
      }
    }
  }
  console.log(`collected ${moved} blessed fixtures`);
  process.exit(0);
}

const authority = [em(0x1f9f0) + "framework", em(0x1f6cd) + "products", em(0x1f4bb) + "os", em(0x1f528) + "modules", em(0x1f5e3) + "dsl", em(0x2728) + "derive", em(0x1f523) + "mutation-authority.json"];
mkdirSync(join(privRepo, ...authority.slice(0, -1)), { recursive: true });
copyFileSync(join(repo, ...authority), join(privRepo, ...authority));
writeFileSync(join(privRepo, "nx.json"), "{}\n");
writeFileSync(join(privRepo, em(0x1f4cb) + "project.json"), "{}\n");
for (const leafDir of leafDirs) {
  copyTree(join(mutations, leafDir), join(privMutations, leafDir));
  copyTree(join(realFixtures, leafDir), join(privFixtures, leafDir));
}
copyTree(join(mutations, em(0x1f9f0) + "kit"), join(privMutations, em(0x1f9f0) + "kit"));

const real = readFileSync(join(artifact, RS), "utf8").replaceAll("\r\n", "\n");
const slice = (from: string, to: string) => {
  const start = real.indexOf(from);
  return real.slice(start, real.indexOf(to, start));
};

const aggregate = readFileSync(join(mutations, RS), "utf8")
  .replaceAll("\r\n", "\n")
  .split("\n")
  .filter((line) => {
    const variant = line.match(/^ {4}[A-Z]\w+\(super::(\w+)::/);
    if (variant) return mine.has(variant[1]);
    const kind = line.match(/^ {4}"([a-z-]+)",$/);
    if (kind) return mine.has(kind[1].replaceAll("-", "_"));
    return true;
  })
  .join("\n")
  .replace(/^#\[path = "(?!🧰️kit)[^"]*"\]\npub mod \w+;\n\n?/gm, "")
  .replace(/#\[cfg\(test\)\]\n#\[path = "🧪️tests\/🔬️unit\/🦀️\.rs"\]\nmod tests;\n?/, "");
writeFileSync(join(privMutations, RS), aggregate);

const lead = '                        #[path = "."]\n';
const blocks = real.split(lead).filter((block) => /^ {24}pub mod \w+ \{/.test(block)).map((block) => lead + block.replace(/ *\/\/#endregion[\s\S]*$/, ""));
const modName = (block: string) => block.match(/pub mod (\w+) \{/)![1];
const schemaBlock = slice('                    #[path = "."]\n                    pub mod snapshot {', '                    #[path = "."]\n                    pub mod inferences {');
const mutationsFull = slice('                    #[path = "."]\n                    pub mod mutations {', "//#region");
const mutationsHead = mutationsFull.slice(0, mutationsFull.indexOf("pub use component::*;") + "pub use component::*;".length) + "\n";
const standards = em(0x1f3c5) + "standards/";
const text = `extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub const BIM_MODEL_DOCUMENT_SCHEMA: &str = "s.bim.model@1";

pub use crate::standards::v1::subsets::any::schema::diff::patches::*;
pub use crate::standards::v1::subsets::any::schema::diff::{Assigned, Entry, KeyedDelta, ModelDiff, Patch, PropertySetPatch};
pub use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::*;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
${schemaBlock.replaceAll(`#[path = "${standards}`, `#[path = "${abs(artifact)}/${standards}`)}${mutationsHead}
${blocks.filter((block) => mine.has(modName(block))).join("")}
                    }
                }
            }
        }
    }
}
`;
const io = join(artifact, em(0x1f3c5) + "standards", em(0x1f516) + "1", em(0x1fa86) + "subsets", em(0x2733) + "any", em(0x1f6aa) + "io");
const realIo = (family: string) => abs(join(io, em(family === "binary" ? 0x1f4be : 0x1f4dd) + family, em(0x1f9ec) + "mutations", RS));
const wire = `
#[cfg(test)]
mod wire {
    use crate::ModelMutation;
    use protocol::OpText;

    #[path = "${realIo("binary")}"]
    mod binary;
    #[path = "${realIo("text")}"]
    mod text;

    #[semio_framework_async_macros::async_test]
    async fn the_slice_round_trips_text_and_binary() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../${em(0x1f3c5)}standards/${em(0x1f516)}1/${em(0x1fa86)}subsets/${em(0x2733)}any/${em(0x1f9eb)}fixtures/${em(0x1f9ec)}mutations");
        let (operations, files) = protocol::mutation_fixture_ops::<ModelMutation>(&root);
        assert_eq!(operations.len(), files);
        assert!(files >= 24, "found {files} fixtures");
        for operation in operations {
            assert_eq!(ModelMutation::parse_op(&operation.print_op()).expect("text op parses"), operation);
            assert_eq!(binary::decode_op(&binary::encode_op(&operation).expect("binary op encodes")).expect("binary op decodes"), operation);
        }
    }
}
`;
writeFileSync(join(privArtifact, RS), text + wire);

const member = join(privArtifact, em(0x1f4e6) + "packages", em(0x1f980) + "rust");
const seedManifest = join(priv, "x", "y", "Cargo.toml");
mkdirSync(member, { recursive: true });
writeFileSync(join(member, "Cargo.toml"), readFileSync(seedManifest, "utf8").replace(/\[lib\]\npath = "[^"]+"/, `[lib]\npath = "${abs(join(privArtifact, RS))}"`));
const rootManifest = join(priv, "Cargo.toml");
writeFileSync(rootManifest, readFileSync(rootManifest, "utf8").replace(/members = \[[^\]]*\]/, `members = ["${abs(relative(priv, member))}"]`));
console.log(`isolated tree: ${mine.size} leaves, ${blocks.filter((block) => mine.has(modName(block))).length} mounted`);
void child;
