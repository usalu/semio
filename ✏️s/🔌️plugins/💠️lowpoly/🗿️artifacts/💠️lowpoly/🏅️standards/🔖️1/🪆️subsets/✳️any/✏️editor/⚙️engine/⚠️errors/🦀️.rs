//! ⚠️ Lowpoly engine errors: the `LowpolyCoreError` vocabulary of the mesh-editing compute session, re-exported by the engine root.
//! See [`std::error::Error`] and [`std::fmt::Display`] for the implemented traits.

use semio_framework_3d::mesh::MeshKernelError;

//#region ⚠️ Errors
/// ⚠️ `LowpolyDocument` compute-session and mesh-mutation / compute-session failure.
#[derive(Debug)]
pub enum LowpolyCoreError {
    Mesh(MeshKernelError),
    Managed(semio_framework_value::ValueError),
    UnknownPrimitive(String),
    LayerIndexOutOfRange,
    NoActiveObject,
    MeshMissing,
    ObjectNotFound,
    /// 🕸️ The session-local `mesh_workspace` cache (`🖌️session::LowpolyScratch`) has no entry for an
    /// an undo/redo of a `create-mesh`/`delete-mesh` (store-level undo/redo bypass `ArtifactApp::handle`
    /// entirely, so this cache is never resynced by them). Fails closed rather than silently editing
    /// stale geometry and re-deriving a WRONG handle from it via `sync_meshes_to_snapshot`.
    StaleMeshWorkspace(String),
}

impl std::fmt::Display for LowpolyCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mesh(error) => write!(formatter, "{error}"),
            Self::Managed(error)=>write!(formatter,"{error:?}"),
            Self::UnknownPrimitive(primitive) => write!(formatter, "unknown primitive: {primitive}"),
            Self::LayerIndexOutOfRange => formatter.write_str("layer index out of range"),
            Self::NoActiveObject => formatter.write_str("no active object"),
            Self::MeshMissing => formatter.write_str("mesh missing"),
            Self::ObjectNotFound => formatter.write_str("object not found"),
            Self::StaleMeshWorkspace(object) => write!(formatter, "mesh workspace missing or stale for object {object}"),
        }
    }
}

impl std::error::Error for LowpolyCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Mesh(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<MeshKernelError> for LowpolyCoreError {
    fn from(error: MeshKernelError) -> Self {
        Self::Mesh(error)
    }
}
//#endregion ⚠️ Errors
