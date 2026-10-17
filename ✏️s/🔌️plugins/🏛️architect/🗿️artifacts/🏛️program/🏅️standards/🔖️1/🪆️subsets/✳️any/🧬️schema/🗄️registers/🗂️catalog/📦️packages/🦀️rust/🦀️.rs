//! 🗂️ catalog of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

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
                    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel;
                    #[path = "."]
                    pub mod registers {
                        pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::*;
                        pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::registers::kernel;
                        #[path = "."]
                        pub mod records {
                            pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::records::*;
                            #[cfg(test)]
                            #[path = "../../../🗃️records/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../🦀️.rs"]
                        pub mod catalog;
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️AdjacencyTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-adjacency/🦀️.rs"]
                        pub mod tests_adjacency;
                        #[cfg(test)]
                        //#region 🧪️ValidateTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-validate/🦀️.rs"]
                        pub mod tests_validate;
                        #[cfg(test)]
                        //#region 🧪️OutputsTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-outputs/🦀️.rs"]
                        pub mod tests_outputs;
                        #[cfg(test)]
                        //#region 🧪️ReportTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-report/🦀️.rs"]
                        pub mod tests_report;
                        #[cfg(test)]
                        //#region 🧪️StatusSummaryTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-status-summary/🦀️.rs"]
                        pub mod tests_status_summary;
                        #[cfg(test)]
                        //#region 🧪️SearchTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-search/🦀️.rs"]
                        pub mod tests_search;
                        #[cfg(test)]
                        //#region 🧪️AnalyzeTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-analyze/🦀️.rs"]
                        pub mod tests_analyze;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️TraceReadsTests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️tests-trace/🦀️.rs"]
                        pub mod tests_trace;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod rename_governance {
                            #[path = "../../../../🧬️mutations/🏛️governance/🏷️rename/🦀️.rs"]
                            mod component;
                            #[path = "../../../../🧬️mutations/🏛️governance/🏷️rename/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🏛️governance/🏷️rename/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
