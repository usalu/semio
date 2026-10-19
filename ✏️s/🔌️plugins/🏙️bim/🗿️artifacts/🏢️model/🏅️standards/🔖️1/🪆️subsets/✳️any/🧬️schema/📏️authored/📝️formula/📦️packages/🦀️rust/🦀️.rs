//! 📝️ formula of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;

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
                    pub mod authored {
                        #[path = "../../🦀️.rs"]
                        pub mod formula;
                        #[path = "../../../🧭️plan/🦀️.rs"]
                        pub mod plan;
                        #[path = "../../../▭️profile/🦀️.rs"]
                        pub mod profile;
                    }
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "."]
                        pub mod property_kit {
                            #[cfg(test)]
                            #[path = "../../../../📸️snapshot/🏷️property-kit/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod conditions {
                            #[cfg(test)]
                            #[path = "../../../../📸️snapshot/🔥️conditions/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod views {
                            #[cfg(test)]
                            #[path = "../../../../📸️snapshot/🖼️views/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod sheets {
                            #[cfg(test)]
                            #[path = "../../../../📸️snapshot/📄️sheets/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod coordination {
                            #[cfg(test)]
                            #[path = "../../../../📸️snapshot/🤝️coordination/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod office {
        #[path = "../../../../../📚️examples/🏢️office/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bim {
        #[path = "."]
        pub mod terminology {
            #[cfg(test)]
            #[path = "../../../../../👁️viewer/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
}
