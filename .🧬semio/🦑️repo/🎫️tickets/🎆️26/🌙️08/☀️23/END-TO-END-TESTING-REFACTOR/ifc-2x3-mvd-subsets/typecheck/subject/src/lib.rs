//! 🧪️ Mounts the REAL production files under a faithful stand-in crate tree so the three IFC2X3
//! MVD mutation modules and the shared 🧬️mvd primitives can be typechecked and unit-tested while
//! the stdio plugin crate itself cannot compile. The `part21` module below is the REAL one, mounted
//! by absolute path; only `Ifc2x3Snapshot`/`Ifc2x3Diff`/`Ifc2x3Mutation` are stand-ins, and
//! `Ifc2x3Diff` is a whole-snapshot delta, which is exactly what these vocabularies produce.

pub mod artifacts {
    pub mod step {
        pub mod engine {
            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📐️part21/🦀️component.rs"]
            pub mod part21;
        }
    }
    pub mod ifc {
        pub mod standards {
            pub mod v2x3 {
                #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🧬️mvd/🦀️component.rs"]
                pub mod mvd;

                pub mod subsets {
                    pub mod any {
                        pub mod schema {
                            pub mod snapshot {
                                use crate::artifacts::step::engine::part21::Part21Document;
                                use serde::{Deserialize, Serialize};

                                #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
                                pub struct Ifc2x3Snapshot {
                                    pub schema: String,
                                    pub document: Part21Document,
                                    pub edm_preamble: Option<String>,
                                }

                                impl Default for Ifc2x3Snapshot {
                                    fn default() -> Self {
                                        Self { schema: "stdio.ifc.2x3".into(), document: Part21Document::default(), edm_preamble: None }
                                    }
                                }

                                pub fn validate_ifc2x3_snapshot(snapshot: &Ifc2x3Snapshot) -> Result<(), String> {
                                    if snapshot.schema != "stdio.ifc.2x3" {
                                        return Err(format!("ifc2x3: unsupported snapshot schema {:?}", snapshot.schema));
                                    }
                                    Ok(())
                                }
                            }
                            pub mod diff {
                                use super::snapshot::Ifc2x3Snapshot;
                                use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};
                                use serde::{Deserialize, Serialize};

                                #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
                                pub struct Ifc2x3Diff {
                                    pub next: Option<Ifc2x3Snapshot>,
                                }

                                impl MutationDiff<Ifc2x3Snapshot> for Ifc2x3Diff {
                                    fn apply(&self, base: &Ifc2x3Snapshot) -> MutationApplyResult<Ifc2x3Snapshot> {
                                        Ok(self.next.clone().unwrap_or_else(|| base.clone()))
                                    }
                                    fn absorb(&mut self, other: Self) {
                                        if other.next.is_some() {
                                            self.next = other.next;
                                        }
                                    }
                                }

                                impl DiffAlgebra<Ifc2x3Snapshot> for Ifc2x3Diff {
                                    fn inverse(&self, base: &Ifc2x3Snapshot) -> Self {
                                        Self { next: Some(base.clone()) }
                                    }
                                    fn between(base: &Ifc2x3Snapshot, other: &Ifc2x3Snapshot) -> Self {
                                        if base == other {
                                            Self { next: None }
                                        } else {
                                            Self { next: Some(other.clone()) }
                                        }
                                    }
                                    fn is_empty(&self) -> bool {
                                        self.next.is_none()
                                    }
                                }
                            }
                            pub mod mutations {
                                use super::diff::Ifc2x3Diff;
                                use super::snapshot::Ifc2x3Snapshot;
                                use serde::{Deserialize, Serialize};

                                #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
                                pub enum Ifc2x3Mutation {
                                    #[default]
                                    NoMutation,
                                }

                                pub fn apply_ifc2x3_mutation(_snapshot: &mut Ifc2x3Snapshot, _mutation: &Ifc2x3Mutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
                                    protocol::MutationOutcome::new(Ifc2x3Diff::default())
                                }
                            }
                        }
                    }
                    pub mod cv20 {
                        pub mod schema {
                            pub mod derived_analysis {
                                pub const FORBIDDEN_STRUCTURAL_TYPES: &[&str] = &["IFCSTRUCTURALANALYSISMODEL", "IFCSTRUCTURALCURVEMEMBER", "IFCSTRUCTURALLOADGROUP"];
                                pub const GEOMETRY_BEARING_PRODUCT_TYPES: &[&str] = &["IFCWALL", "IFCWALLSTANDARDCASE", "IFCDOOR", "IFCWINDOW", "IFCSLAB", "IFCBEAM", "IFCCOLUMN", "IFCROOF", "IFCSTAIR", "IFCBUILDINGELEMENTPROXY"];
                            }
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧬️schema/🧬️mutations/🦀️component.rs"]
                            pub mod mutations;
                        }
                    }
                    pub mod cobie {
                        pub mod schema {
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧬️schema/🧬️mutations/🦀️component.rs"]
                            pub mod mutations;
                        }
                    }
                    pub mod sav {
                        pub mod schema {
                            #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧬️schema/🧬️mutations/🦀️component.rs"]
                            pub mod mutations;
                        }
                    }
                }
            }
        }
    }
}
