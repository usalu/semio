#!/usr/bin/env bun
/**
 * 🔬️ Isolated build of the inference tree while the full crate does not compile (the `MutationDiff::apply` refactor is propagating through the stdio crates the
 * artifact crate links). Writes a private lib root into `🗑️generated/i-solids-walls/priv/m/lib.rs` made of the REAL `snapshot`, `diff` and `inferences` mount blocks of
 * the artifact root (extracted textually, so every family module of every agent is included) plus a two-kind mutation stub for the tests that apply storey and wall
 * edits. All sources are compiled in place through the `🏅️standards` junction. Run `bun 🗑️generated/i-solids-walls/priv-setup.ts` first.
 * Usage: `gate.sh i-solids-walls -- cargo test --manifest-path 🗑️generated/i-solids-walls/priv/Cargo.toml -p semio-s-artifact-bim-model --lib element_solids`.
 */
import { existsSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { artifact, subsets } from "./r3-f1-paths.ts";

const priv = join(import.meta.dir, "🗑️generated", "i-solids-walls", "priv");
const standards = dirname(subsets);
mkdirSync(join(priv, "m"), { recursive: true });
const junction = join(priv, "m", basename(standards));
if (!existsSync(junction)) symlinkSync(standards, junction, "junction");
const root = readFileSync(join(artifact, "🦀️.rs"), "utf8");

const block = (header: string): string => {
  const start = root.indexOf(header);
  if (start < 0) throw new Error(`missing ${header}`);
  const open = root.indexOf("{", start);
  let depth = 0;
  for (let at = open; at < root.length; at++) {
    if (root[at] === "{") depth++;
    if (root[at] === "}" && --depth === 0) {
      const lineStart = root.lastIndexOf("\n", start) + 1;
      const before = root.lastIndexOf("\n", lineStart - 2) + 1;
      const attribute = root.slice(before, lineStart).trim().startsWith("#[path") ? before : lineStart;
      return root.slice(attribute, at + 1) + "\n";
    }
  }
  throw new Error(`unbalanced ${header}`);
};

const text = `extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::diff::patches::*;
pub use crate::standards::v1::subsets::any::schema::diff::{Assigned, Entry, KeyedDelta, ModelDiff, Patch, PropertySetPatch};
pub use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
pub use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::*;

pub const BIM_MODEL_DOCUMENT_SCHEMA: &str = "s.bim.model@1";

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub mod io {
                    pub mod text {
                        pub mod snapshot {
                            pub fn encode_inference_projection_json(_snapshot: &crate::ModelSnapshot, _table: &str) -> Option<String> {
                                None
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
${block("pub mod snapshot {")}${block("pub mod diff {")}${block("pub mod inferences {")}
                    pub mod mutations {
                        pub const KINDS: &[&str] = &[];
                        pub mod set_storey_height {
                            pub struct SetStoreyHeight {
                                pub id: String,
                                pub height: f64,
                            }
                        }
                        pub mod set_wall_top {
                            pub struct SetWallTop {
                                pub id: String,
                                pub top: crate::TopConstraint,
                            }
                        }
                        pub enum ModelMutation {
                            SetStoreyHeight(set_storey_height::SetStoreyHeight),
                            SetWallTop(set_wall_top::SetWallTop),
                        }
                        pub fn apply_model_mutation(base: &crate::ModelSnapshot, mutation: &ModelMutation) -> Result<crate::ModelSnapshot, String> {
                            let diff = match mutation {
                                ModelMutation::SetStoreyHeight(row) => crate::ModelDiff::storeys(row.id.clone(), crate::Entry::Patched(crate::StoreyPatch { height: Some(row.height), ..Default::default() })),
                                ModelMutation::SetWallTop(row) => crate::ModelDiff::walls(row.id.clone(), crate::Entry::Patched(crate::WallPatch { top: Some(row.top.clone()), ..Default::default() })),
                            };
                            protocol::apply_diff(&diff, base).map_err(|error| error.message.to_string())
                        }
                    }
                }
            }
        }
    }
}
`;
const aliased = ["stair_runs", "spaces", "quantities"].reduce((out, name) => out.replace(new RegExp(`pub mod ${name} \{`), `pub mod ${name} { pub use super::storey_levels;`), text);
writeFileSync(join(priv, "m", "lib.rs"), aliased);
const manifest = join(priv, "x", "y", "Cargo.toml");
writeFileSync(manifest, readFileSync(manifest, "utf8").replace(/\[lib\]\npath = "[^"]+"/, `[lib]\npath = "${join(priv, "m", "lib.rs").replaceAll("\\", "/")}"`));
console.log("isolated lib written");
