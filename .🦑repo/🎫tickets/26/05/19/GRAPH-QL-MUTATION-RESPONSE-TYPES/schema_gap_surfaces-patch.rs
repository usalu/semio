//#region 🩹 schema_gap_surfaces

pub mod schema_gap_surfaces {
    //! 🩹 SDL-only synthetic relay surfaces for long-tail golden declarations; registered into `Schema::sdl()` so the exported schema reaches the current target declaration set.

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
        (@apply_families) => {
            gap_surface_families! {
        AddedAttributeToConcept,
        AddedAttributeToDesign,
        AddedAttributeToDesignInput,
        AddedAttributeToPiece,
        AddedAttributeToPieceInput,
        AddedAttributeToPort,
        AddedAttributeToQuality,
        AddedAttributeToTag,
        AddedAttributeToType,
        AddedAttributeToTypeInput,
        AddedAttributesToConcept,
        AddedAttributesToDesign,
        AddedAttributesToDesignInput,
        AddedAttributesToPiece,
        AddedAttributesToPieceInput,
        AddedAttributesToPort,
        AddedAttributesToQuality,
        AddedAttributesToTag,
        AddedAttributesToType,
        AddedAttributesToTypeInput,
        AddedChildPieceWithParentConnection,
        AddedChildPieceWithParentConnectionInput,
        AddedChildPiecesWithParentConnections,
        AddedChildPiecesWithParentConnectionsInput,
        AddedConnector,
        AddedConnectorInput,
        AddedConnectors,
        AddedConnectorsInput,
        AddedHangingChildPieceWithParentConnectionInput,
        AddedHangingChildPieceWithParentConnection,
        AddedHangingChildPiecesWithParentConnections,
        AddedHangingChildPiecesWithParentConnectionsInput,
        AttributeDiff,
        AttributeModification,
        AttributeModifications,
        AuthorDiff,
        AuthorModification,
        AuthorModifications,
        BenchmarkDiff,
        BenchmarkModification,
        BenchmarkModifications,
        ChangedPieceToType,
        ChangedPieceToTypeInput,
        ChangedPiecesToType,
        ChangedPiecesToTypeInput,
        ConceptDiff,
        ConceptModification,
        ConceptModifications,
        ConceptOperation,
        ConnectionDiff,
        ConnectionModification,
        ConnectionModifications,
        ConnectorDiff,
        ConnectorModification,
        ConnectorModifications,
        ConnectorOperation,
        CreatedConcept,
        CreatedConcepts,
        CreatedDesign,
        CreatedDesignInput,
        CreatedDesigns,
        CreatedDesignsInput,
        CreatedPort,
        CreatedPorts,
        CreatedQualities,
        CreatedQuality,
        CreatedTag,
        CreatedTags,
        CreatedType,
        CreatedTypeInput,
        CreatedTypes,
        CreatedTypesInput,
        DeletedConcept,
        DeletedConcepts,
        DeletedDesign,
        DeletedDesigns,
        DeletedPiece,
        DeletedPieces,
        DeletedPiecesAndConnections,
        DeletedPort,
        DeletedPorts,
        DeletedQualities,
        DeletedQuality,
        DeletedTag,
        DeletedTags,
        DeletedType,
        DeletedTypes,
        DesignModification,
        DesignModifications,
        DesignOperation,
        DraggedPieces,
        DraggedPiecesInput,
        FamilyDiff,
        FamilyModification,
        FamilyModifications,
        FileDiff,
        FileModification,
        FileModifications,
        FixedPieces,
        FlattenedDesign,
        FolderDiff,
        FolderModification,
        FolderModifications,
        GroupDiff,
        GroupModification,
        GroupModifications,
        KitModification,
        KitModifications,
        KitOperation,
        LayerDiff,
        LayerModification,
        LayerModifications,
        MovedPiece,
        MovedPieceInput,
        MovedPieces,
        MovedPiecesInput,
        PieceDiff,
        PieceModification,
        PieceModifications,
        PieceOperation,
        PiecesOperation,
        PlaceDiff,
        PlaceModification,
        PlaceModifications,
        PortDiff,
        PortModification,
        PortModifications,
        PortOperation,
        PropDiff,
        PropModification,
        PropModifications,
        QualityDiff,
        QualityModification,
        QualityModifications,
        QualityOperation,
        RemovedAttributeFromConcept,
        RemovedAttributeFromDesign,
        RemovedAttributeFromPiece,
        RemovedAttributeFromPort,
        RemovedAttributeFromQuality,
        RemovedAttributeFromTag,
        RemovedAttributeFromType,
        RemovedAttributesFromConcept,
        RemovedAttributesFromDesign,
        RemovedAttributesFromPiece,
        RemovedAttributesFromPort,
        RemovedAttributesFromQuality,
        RemovedAttributesFromTag,
        RemovedAttributesFromType,
        RemovedConnector,
        RemovedConnectors,
        RenamedConcept,
        RenamedConnector,
        RenamedConnectorInput,
        RenamedPiece,
        RenamedPieceInput,
        RenamedPort,
        RenamedQuality,
        RenamedTag,
        RenamedType,
        RenamedTypeInput,
        RepresentationDiff,
        RepresentationModification,
        RepresentationModifications,
        SideDiff,
        SideModification,
        SideModifications,
        StatDiff,
        StatModification,
        StatModifications,
        TagDiff,
        TagModification,
        TagModifications,
        TagOperation,
        TypeDiff,
        TypeModification,
        TypeModifications,
        TypeOperation,
        UpdatedConceptDescription,
        UpdatedConceptIcon,
        UpdatedConnectorDescription,
        UpdatedConnectorDescriptionInput,
        UpdatedConnectorIcon,
        UpdatedConnectorIconInput,
        UpdatedPieceDescription,
        UpdatedPieceDescriptionInput,
        UpdatedPortDescription,
        UpdatedPortIcon,
        UpdatedQualityDescription,
        UpdatedQualityIcon,
        UpdatedTagDescription,
        UpdatedTagIcon,
        UpdatedTypeDescription,
        UpdatedTypeDescriptionInput,
        UpdatedTypeIcon,
        UpdatedTypeIconInput
            }
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_family_connections!($builder,
        AddedAttributeToConcept,
        AddedAttributeToDesign,
        AddedAttributeToDesignInput,
        AddedAttributeToPiece,
        AddedAttributeToPieceInput,
        AddedAttributeToPort,
        AddedAttributeToQuality,
        AddedAttributeToTag,
        AddedAttributeToType,
        AddedAttributeToTypeInput,
        AddedAttributesToConcept,
        AddedAttributesToDesign,
        AddedAttributesToDesignInput,
        AddedAttributesToPiece,
        AddedAttributesToPieceInput,
        AddedAttributesToPort,
        AddedAttributesToQuality,
        AddedAttributesToTag,
        AddedAttributesToType,
        AddedAttributesToTypeInput,
        AddedChildPieceWithParentConnection,
        AddedChildPieceWithParentConnectionInput,
        AddedChildPiecesWithParentConnections,
        AddedChildPiecesWithParentConnectionsInput,
        AddedConnector,
        AddedConnectorInput,
        AddedConnectors,
        AddedConnectorsInput,
        AddedHangingChildPieceWithParentConnectionInput,
        AddedHangingChildPieceWithParentConnection,
        AddedHangingChildPiecesWithParentConnections,
        AddedHangingChildPiecesWithParentConnectionsInput,
        AttributeDiff,
        AttributeModification,
        AttributeModifications,
        AuthorDiff,
        AuthorModification,
        AuthorModifications,
        BenchmarkDiff,
        BenchmarkModification,
        BenchmarkModifications,
        ChangedPieceToType,
        ChangedPieceToTypeInput,
        ChangedPiecesToType,
        ChangedPiecesToTypeInput,
        ConceptDiff,
        ConceptModification,
        ConceptModifications,
        ConceptOperation,
        ConnectionDiff,
        ConnectionModification,
        ConnectionModifications,
        ConnectorDiff,
        ConnectorModification,
        ConnectorModifications,
        ConnectorOperation,
        CreatedConcept,
        CreatedConcepts,
        CreatedDesign,
        CreatedDesignInput,
        CreatedDesigns,
        CreatedDesignsInput,
        CreatedPort,
        CreatedPorts,
        CreatedQualities,
        CreatedQuality,
        CreatedTag,
        CreatedTags,
        CreatedType,
        CreatedTypeInput,
        CreatedTypes,
        CreatedTypesInput,
        DeletedConcept,
        DeletedConcepts,
        DeletedDesign,
        DeletedDesigns,
        DeletedPiece,
        DeletedPieces,
        DeletedPiecesAndConnections,
        DeletedPort,
        DeletedPorts,
        DeletedQualities,
        DeletedQuality,
        DeletedTag,
        DeletedTags,
        DeletedType,
        DeletedTypes,
        DesignModification,
        DesignModifications,
        DesignOperation,
        DraggedPieces,
        DraggedPiecesInput,
        FamilyDiff,
        FamilyModification,
        FamilyModifications,
        FileDiff,
        FileModification,
        FileModifications,
        FixedPieces,
        FlattenedDesign,
        FolderDiff,
        FolderModification,
        FolderModifications,
        GroupDiff,
        GroupModification,
        GroupModifications,
        KitModification,
        KitModifications,
        KitOperation,
        LayerDiff,
        LayerModification,
        LayerModifications,
        MovedPiece,
        MovedPieceInput,
        MovedPieces,
        MovedPiecesInput,
        PieceDiff,
        PieceModification,
        PieceModifications,
        PieceOperation,
        PiecesOperation,
        PlaceDiff,
        PlaceModification,
        PlaceModifications,
        PortDiff,
        PortModification,
        PortModifications,
        PortOperation,
        PropDiff,
        PropModification,
        PropModifications,
        QualityDiff,
        QualityModification,
        QualityModifications,
        QualityOperation,
        RemovedAttributeFromConcept,
        RemovedAttributeFromDesign,
        RemovedAttributeFromPiece,
        RemovedAttributeFromPort,
        RemovedAttributeFromQuality,
        RemovedAttributeFromTag,
        RemovedAttributeFromType,
        RemovedAttributesFromConcept,
        RemovedAttributesFromDesign,
        RemovedAttributesFromPiece,
        RemovedAttributesFromPort,
        RemovedAttributesFromQuality,
        RemovedAttributesFromTag,
        RemovedAttributesFromType,
        RemovedConnector,
        RemovedConnectors,
        RenamedConcept,
        RenamedConnector,
        RenamedConnectorInput,
        RenamedPiece,
        RenamedPieceInput,
        RenamedPort,
        RenamedQuality,
        RenamedTag,
        RenamedType,
        RenamedTypeInput,
        RepresentationDiff,
        RepresentationModification,
        RepresentationModifications,
        SideDiff,
        SideModification,
        SideModifications,
        StatDiff,
        StatModification,
        StatModifications,
        TagDiff,
        TagModification,
        TagModifications,
        TagOperation,
        TypeDiff,
        TypeModification,
        TypeModifications,
        TypeOperation,
        UpdatedConceptDescription,
        UpdatedConceptIcon,
        UpdatedConnectorDescription,
        UpdatedConnectorDescriptionInput,
        UpdatedConnectorIcon,
        UpdatedConnectorIconInput,
        UpdatedPieceDescription,
        UpdatedPieceDescriptionInput,
        UpdatedPortDescription,
        UpdatedPortIcon,
        UpdatedQualityDescription,
        UpdatedQualityIcon,
        UpdatedTagDescription,
        UpdatedTagIcon,
        UpdatedTypeDescription,
        UpdatedTypeDescriptionInput,
        UpdatedTypeIcon,
        UpdatedTypeIconInput
            )
        };
    }

    #[macro_export]
    macro_rules! gap_surface_existing_relay_name_list {
        (@apply_relays) => {
            gap_surface_existing_relays! {
        AddedAttributeToConceptInput,
        AddedAttributeToPortInput,
        AddedAttributeToQualityInput,
        AddedAttributeToTagInput,
        AddedAttributesToConceptInput,
        AddedAttributesToPortInput,
        AddedAttributesToQualityInput,
        AddedAttributesToTagInput,
        AlternativeCommand,
        ChangedDescription,
        CreatedConceptInput,
        CreatedConceptsInput,
        CreatedFixedPiece,
        CreatedPortInput,
        CreatedPortsInput,
        CreatedQualitiesInput,
        CreatedQualityInput,
        CreatedTagInput,
        CreatedTagsInput,
        DraggedPiece,
        FileBackbone,
        FileBackboneCommand,
        FixedPiece,
        Graph,
        Kit,
        LocalProviderCommand,
        Place,
        RemoteProviderCommand,
        RenamedConceptInput,
        RenamedKit,
        RenamedPortInput,
        RenamedQualityInput,
        RenamedTagInput,
        Session,
        SessionCommand,
        Side,
        StoreCommand,
        TheKit,
        UnsavedChangeCommand,
        UpdatedConceptDescriptionInput,
        UpdatedConceptIconInput,
        UpdatedPortDescriptionInput,
        UpdatedPortIconInput,
        UpdatedQualityDescriptionInput,
        UpdatedQualityIconInput,
        UpdatedTagDescriptionInput,
        UpdatedTagIconInput,
        VersionCommand,
        WebsocketBackbone,
        WebsocketBackboneCommand
            }
        };
        (@register $builder:expr) => {
            $crate::register_gap_surface_existing_relay_connections!($builder,
        AddedAttributeToConceptInput,
        AddedAttributeToPortInput,
        AddedAttributeToQualityInput,
        AddedAttributeToTagInput,
        AddedAttributesToConceptInput,
        AddedAttributesToPortInput,
        AddedAttributesToQualityInput,
        AddedAttributesToTagInput,
        AlternativeCommand,
        ChangedDescription,
        CreatedConceptInput,
        CreatedConceptsInput,
        CreatedFixedPiece,
        CreatedPortInput,
        CreatedPortsInput,
        CreatedQualitiesInput,
        CreatedQualityInput,
        CreatedTagInput,
        CreatedTagsInput,
        DraggedPiece,
        FileBackbone,
        FileBackboneCommand,
        FixedPiece,
        Graph,
        Kit,
        LocalProviderCommand,
        Place,
        RemoteProviderCommand,
        RenamedConceptInput,
        RenamedKit,
        RenamedPortInput,
        RenamedQualityInput,
        RenamedTagInput,
        Session,
        SessionCommand,
        Side,
        StoreCommand,
        TheKit,
        UnsavedChangeCommand,
        UpdatedConceptDescriptionInput,
        UpdatedConceptIconInput,
        UpdatedPortDescriptionInput,
        UpdatedPortIconInput,
        UpdatedQualityDescriptionInput,
        UpdatedQualityIconInput,
        UpdatedTagDescriptionInput,
        UpdatedTagIconInput,
        VersionCommand,
        WebsocketBackbone,
        WebsocketBackboneCommand
            )
        };
    }

    #[macro_export]
    macro_rules! with_gap_surface_family_names {
        (gap_surface_families) => {
            $crate::gap_surface_family_name_list!(@apply_families);
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

    with_gap_surface_family_names!(gap_surface_families);

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
            $crate::gap_surface_existing_relay_name_list!(@apply_relays);
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

    with_gap_surface_existing_relay_names!(gap_surface_existing_relays);

}

//#endregion schema_gap_surfaces
