import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/compose/compose/client/lib/rs/lib.rs";
const src = readFileSync(path, "utf8");
const modStart = src.indexOf("pub mod schema_gap_surfaces");
if (modStart < 0) {
  console.error("pub mod schema_gap_surfaces not found");
  process.exit(1);
}
const regionLineStart = src.lastIndexOf("//#region", modStart);
const regionEnd = src.indexOf("//#region", modStart + 1);
if (regionLineStart < 0 || regionEnd < 0) {
  console.error("region bounds not found (expected meta //#region after schema_gap_surfaces)");
  process.exit(1);
}

const chunk = src.slice(regionLineStart, regionEnd);
const parseNames = (macroName) => {
  const re = new RegExp(`macro_rules! ${macroName}[\\s\\S]*?\\(@names\\) => \\{([\\s\\S]*?)\\n        \\};`);
  const m = chunk.match(re);
  if (!m) throw new Error(`names for ${macroName} not found`);
  return [...m[1].matchAll(/^\s+([A-Za-z][A-Za-z0-9]*),?$/gm)].map((x) => x[1]);
};

const headPath = "c:/git/compose/.repo/🎫️/26/05/19/GRAPH-QL-MUTATION-RESPONSE-TYPES/lib-head.rs";
const head = readFileSync(headPath, "utf8");
const headMod = head.indexOf("pub mod schema_gap_surfaces");
const headRegionStart = head.lastIndexOf("//#region", headMod);
const headRegionEnd = head.indexOf("//#endregion", headMod);
const headChunk = head.slice(headRegionStart, headRegionEnd);

const parseNamesFromChunk = (source, macroName) => {
  const re = new RegExp(`macro_rules! ${macroName}[\\s\\S]*?\\(@names\\) => \\{([\\s\\S]*?)\\n        \\};`);
  const m = source.match(re);
  if (!m) throw new Error(`names for ${macroName} not found`);
  return [...m[1].matchAll(/^\s+([A-Za-z][A-Za-z0-9]*),?$/gm)].map((x) => x[1]);
};

let familyNames = [];
let relayNames = [];
try {
  familyNames = parseNamesFromChunk(chunk, "gap_surface_family_name_list");
  relayNames = parseNamesFromChunk(chunk, "gap_surface_existing_relay_name_list");
} catch {
  familyNames = parseNamesFromChunk(headChunk, "gap_surface_family_name_list");
  relayNames = parseNamesFromChunk(headChunk, "gap_surface_existing_relay_name_list");
}
if (familyNames.length === 0 || relayNames.length === 0) {
  familyNames = parseNamesFromChunk(headChunk, "gap_surface_family_name_list");
  relayNames = parseNamesFromChunk(headChunk, "gap_surface_existing_relay_name_list");
}
const identLines = (names) => names.map((n) => `        ${n},`).join("\n");

const module = `//#region 🩹️ schema_gap_surfaces

pub mod schema_gap_surfaces {
    //! 🩹️ SDL-only synthetic relay surfaces for long-tail golden declarations; registered into \`Schema::sdl()\` so the exported schema reaches the current target declaration set.

    use std::sync::Arc;

    use async_graphql::SimpleObject;

    use crate::gql_relay::PageInfo;

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

    macro_rules! gap_surface_families {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_family!($Name);)+
        };
    }

    macro_rules! gap_surface_existing_relays {
        { $($Name:ident),* $(,)? } => {
            $(gap_surface_existing_relay!($Name);)+
        };
    }

    #[macro_export]
    macro_rules! gap_surface_family_name_list {
        (@names) => {
${identLines(familyNames)}
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_family_connections!(
                $builder,
                ${familyNames.join(", ")}
            )
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {
        (@names) => {
${identLines(relayNames)}
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections!(
                $builder,
                ${relayNames.join(", ")}
            )
        };
    }

    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::schema_gap_surfaces::gap_surface_families! {
                ${familyNames.join(", ")}
            }
        };
        (register_gap_surface_family_connections, $builder:expr) => {
            $crate::gap_surface_family_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_family_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }

    gap_surface_families! {
${identLines(familyNames)}
    }

    gap_surface_family_named!(
        "ChangedDescriptionInput",
        GapChangedDescriptionInput,
        "ChangedDescriptionInputEdge",
        GapChangedDescriptionInputEdge,
        "ChangedDescriptionInputConnection",
        GapChangedDescriptionInputConnection
    );
    gap_surface_family_named!("Clump", GapClump, "ClumpEdge", GapClumpEdge, "ClumpConnection", GapClumpConnection);
    gap_surface_family_named!(
        "CreatedFixedPieceInput",
        GapCreatedFixedPieceInput,
        "CreatedFixedPieceInputEdge",
        GapCreatedFixedPieceInputEdge,
        "CreatedFixedPieceInputConnection",
        GapCreatedFixedPieceInputConnection
    );
    gap_surface_family_named!("DesignDiff", GapDesignDiff, "DesignDiffEdge", GapDesignDiffEdge, "DesignDiffConnection", GapDesignDiffConnection);
    gap_surface_family_named!(
        "DraggedPieceInput",
        GapDraggedPieceInput,
        "DraggedPieceInputEdge",
        GapDraggedPieceInputEdge,
        "DraggedPieceInputConnection",
        GapDraggedPieceInputConnection
    );
    gap_surface_family_named!("KitDiff", GapKitDiff, "KitDiffEdge", GapKitDiffEdge, "KitDiffConnection", GapKitDiffConnection);
    gap_surface_family_named!(
        "RenamedKitInput",
        GapRenamedKitInput,
        "RenamedKitInputEdge",
        GapRenamedKitInputEdge,
        "RenamedKitInputConnection",
        GapRenamedKitInputConnection
    );
    gap_surface_family_named!("Version", GapVersion, "VersionEdge", GapVersionEdge, "VersionConnection", GapVersionConnection);

    #[macro_export]
    macro_rules! with_gap_surface_existing_relay_names {
        (gap_surface_existing_relays) => {
            $crate::schema_gap_surfaces::gap_surface_existing_relays! {
                ${relayNames.join(", ")}
            }
        };
        (register_gap_surface_existing_relay_connections, $builder:expr) => {
            $crate::gap_surface_existing_relay_name_list!(@register $builder)
        };
    }

    #[macro_export]
    macro_rules! register_gap_surface_existing_relay_connections {
        ($builder:expr, $($Name:ident),+ $(,)?) => {{
            let mut b = $builder;
            $( b = b.register_output_type::<::paste::paste! { $crate::schema_gap_surfaces::[<$Name Connection>] }>(); )+
            b
        }};
    }

    gap_surface_existing_relays! {
${identLines(relayNames)}
    }
}

//#endregion 🩹️ schema_gap_surfaces

`;

const out = src.slice(0, regionLineStart) + module + "\n" + src.slice(regionEnd);
writeFileSync(path, out, "utf8");
console.log("rebuilt schema_gap_surfaces", { families: familyNames.length, relays: relayNames.length });
