//! 🖥️ Plugin-based OS kernel: hot-swappable WASM components, media graph, document VCS.

pub mod host;
pub mod instance;
pub mod media_graph;
pub mod registry;

pub use host::{
    apply_os_operation, create_empty_os_document, create_os_studio, default_os_projection,
    delete_os_studio, import_os_studio_from_json, list_os_studio_catalog_entries,
    load_os_studio_document, materialize_os_projection, os_document_from_json, os_document_to_json,
    seed_os_studio_catalog_if_empty, DevJsonBackbone, LoadedPlugin, LocalJsonBackbone, OsBackboneRef,
    OsConflict, OsDiff, OsDocument, OsEnvelope, OsOp, OsProjection, OsStore, OsVcs, PluginHost,
    PluginHotSwapEvent, RemoteOsBackbone, OS_HOME_VFS_ROOT_ID, OS_STUDIO_BACKBONE_URI_PREFIX,
};
pub use instance::{
    apply_parameter_values_to_projection, create_default_os_parameter, create_os_id,
    is_parameter_port_id, media_port_id_for_spec, media_port_spec_id, os_parameter_types_compatible,
    os_parameter_value, parameter_id_from_port_id, parameter_port_id, patch_os_parameter,
    resolve_parameter_values_for_instance, set_json_pointer_value, materialize_os_app_instance_document_json,
    register_os_fixture_json, resolve_os_source_document, OsAppInstance, OsInstanceState,
    OsParameter, OsParameterFieldBinding, OsParameterFieldSpec, OsParameterType, OsSourceDocument,
    OS_PARAMETER_PORT_PREFIX,
};
pub use media_graph::{
    assert_os_media_export_coverage, empty_media_graph, export_os_app_instance_media,
    list_os_media_graph_vfs_children, media_graph_node_for_instance, os_media_export_extension_for_format,
    os_media_graph_to_flow_fixture, os_media_graph_vfs_export_id, os_media_graph_vfs_instance_folder_id,
    build_os_media_flow_operator_infos, OsMediaFlowOperatorInfo,
    os_media_graph_vfs_instance_id, os_media_graph_vfs_schema, os_media_graph_vfs_source_id,
    os_media_neuron_kind_for_node, register_os_media_export_handler, required_os_media_export_formats,
    sync_media_graph_parameter_ports, validate_media_graph, MediaGraphPosition, MediaGraphValidation,
    OsMediaExportFormat, OsMediaExportResult, OsMediaGraph, OsMediaGraphEdge, OsMediaGraphNode,
    OsMediaGraphVfsNodeRecord, OsMediaGraphVfsSchema, OsMediaPort, ProgramRegistry,
    OS_MEDIA_FLOW_MODULE_ID, OS_MEDIA_GRAPH_SCHEMA, OS_MEDIA_GRAPH_VFS_ROOT_ID, OS_STUDIO_SCHEMA,
};
pub use registry::{
    list_os_programs, list_os_resource_descriptors, merge_os_program_definition, os_app_primary_output_kind,
    os_app_registration, os_baseline_resource, os_in_port, os_out_port, os_program_by_id,
    os_resource_descriptor, register_os_builtin_program, register_os_program_definition,
    resolve_os_app_definition, resources_compatible, seed_os_program_registry_from_resource_map,
    OsAppRegistration, OsAppResourceSpec, OsPlatformAppInput, OsPlatformInput, OsPortSpec,
    OsProgramDefinition, OsResourceDescriptor, OsResourceKindId, PluginRegistry,
    OS_RESOURCE_KIND_IDS,
};
pub use semio_framework_core::*;
pub use vcs::{Author, Checkpoint, DocumentBackboneRef, DocumentVcsCommand, MemoryBackbonePort, VcsError};
