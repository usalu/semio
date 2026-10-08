#!/usr/bin/env bun
/**
 * 🔬️ Isolated build of `opening-frames`: while sibling agents' editor/io/inference work is in flight and the crate does not compile as a whole, this writes a private lib root
 * (snapshot + diff + the four inference modules `opening-frames` depends on) into `🗑️generated/i-openings/priv/m/lib.rs` and points the private manifest at it. All sources are
 * compiled in place through the `🏅️standards` junction. Run `bun 🗑️generated/i-openings/priv-setup.ts` first.
 * Usage: `gate.sh i-openings -- cargo test --manifest-path <priv>/Cargo.toml -p semio-s-artifact-bim-model --lib`.
 */
import { mkdirSync, readFileSync, symlinkSync, writeFileSync, existsSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { subsets } from "./r3-f1-paths.ts";

const priv = join(import.meta.dir, "🗑️generated", "i-openings", "priv");
const standards = dirname(subsets);
mkdirSync(join(priv, "m"), { recursive: true });
const junction = join(priv, "m", basename(standards));
if (!existsSync(junction)) symlinkSync(standards, junction, "junction");
const S = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
const leaf = (name: string, file: string, extra = "") => `#[path = "."]
pub mod ${name} {
${extra}    #[path = "${S}/💡️inferences/${file}/🦀️.rs"]
    mod component;
    pub use component::*;
}
`;
const text = `extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::diff::patches::*;
pub use crate::standards::v1::subsets::any::schema::diff::{Assigned, Entry, KeyedDelta, ModelDiff, Patch, PropertySetPatch};
pub use crate::standards::v1::subsets::any::schema::snapshot::*;
pub use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;

pub const BIM_MODEL_DOCUMENT_SCHEMA: &str = "s.bim.model@1";

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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "${S}/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "${S}/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    pub mod mutations {
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
                    #[path = "."]
                    pub mod inferences {
${[leaf("storey_levels", "🪜️storey-levels"), leaf("wall_layout", "🧱️wall-layout"), leaf("curtain_layout", "🪞️curtain-layout", "    pub use super::wall_layout;\n"), leaf("opening_frames", "🪟️opening-frames")].join("")}
                    }
                }
            }
        }
    }
}
`;
writeFileSync(join(priv, "m", "lib.rs"), text);
const manifest = join(priv, "x", "y", "Cargo.toml");
writeFileSync(manifest, readFileSync(manifest, "utf8").replace(/\[lib\]\npath = "[^"]+"/, `[lib]\npath = "${join(priv, "m", "lib.rs").replaceAll("\\", "/")}"`));
console.log("isolated lib written");
