pub mod schema_gap_surfaces {
    //! 🩹 SDL-only synthetic relay surfaces for long-tail golden declarations; registered into `Schema::sdl()` so the exported schema reaches the current target declaration set.

    use std::sync::Arc;

    use async_graphql::SimpleObject;

    use crate::gql_relay::PageInfo;

    #[macro_export]
    macro_rules! gap_surface_family {
        ($Name:ident) => {
            #[derive(Clone, Debug, Default, SimpleObject)]
            pub struct $Name {
                pub hash: String,
            }

            paste::paste! {
                #[derive(Clone, Debug, Default, SimpleObject)]
                pub struct [<$Name Edge>] {
                    pub cursor: String,
                    pub node: $Name,
                }

                #[derive(Clone, Debug, SimpleObject)]
                pub struct [<$Name Connection>] {
                    pub edges: Vec<[<$Name Edge>]>,
                    #[graphql(name = "pageInfo")]
                    pub page_info: Arc<PageInfo>,
                    pub hash: String,
                }

                impl Default for [<$Name Connection>] {
                    fn default() -> Self {
                        Self {
                            edges: Vec::new(),
                            page_info: Arc::new(PageInfo::default()),
                            hash: String::new(),
                        }
                    }
                }
            }
        };
    }

    macro_rules! gap_surface_family_named {
        (
            $base_name:literal,
            $BaseRust:ident,
            $edge_name:literal,
            $EdgeRust:ident,
            $conn_name:literal,
            $ConnRust:ident
        ) => {
            #[derive(Clone, Debug, Default, SimpleObject)]
            #[graphql(name = $base_name)]
            pub struct $BaseRust {
                pub hash: String,
            }

            #[derive(Clone, Debug, Default, SimpleObject)]
            #[graphql(name = $edge_name)]
            pub struct $EdgeRust {
                pub cursor: String,
                pub node: $BaseRust,
            }

            #[derive(Clone, Debug, SimpleObject)]
            #[graphql(name = $conn_name)]
            pub struct $ConnRust {
                pub edges: Vec<$EdgeRust>,
                #[graphql(name = "pageInfo")]
                pub page_info: Arc<PageInfo>,
                pub hash: String,
            }

            impl Default for $ConnRust {
                fn default() -> Self {
                    Self {
                        edges: Vec::new(),
                        page_info: Arc::new(PageInfo::default()),
                        hash: String::new(),
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay {
        ($Base:ident) => {
            paste::paste! {
                #[derive(Clone, Debug, Default, SimpleObject)]
                pub struct [<$Base Edge>] {
                    pub cursor: String,
                    pub hash: String,
                }

                #[derive(Clone, Debug, SimpleObject)]
                pub struct [<$Base Connection>] {
                    pub edges: Vec<[<$Base Edge>]>,
                    #[graphql(name = "pageInfo")]
                    pub page_info: Arc<PageInfo>,
                    pub hash: String,
                }

                impl Default for [<$Base Connection>] {
                    fn default() -> Self {
                        Self {
                            edges: Vec::new(),
                            page_info: Arc::new(PageInfo::default()),
                            hash: String::new(),
                        }
                    }
                }
            }
        };
    }

    macro_rules! define_gap_surface_families_from_list {
        ($($Name:ident),+ $(,)?) => {
            gap_surface_families! { $($Name),+ }
        };
    }

    macro_rules! define_gap_surface_existing_relays_from_list {
        ($($Name:ident),+ $(,)?) => {
            gap_surface_existing_relays! { $($Name),+ }
        };
    }

