//! 🧶️ Companion tests of os-directory-client: client unit tests, native transports, document HTTP and identity.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_directory_client::*;

#[path = "."]
pub mod client {
    pub use semio_framework_os_directory_client::client::*;
    use semio_framework_os_directory_client::schema::{lease_fields_from_plan_v1, same_lease_fields_v1, DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEventPageV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationDocumentWindowV1, DirectorySpaceAdministrationInviteWindowV1, DirectorySpaceAdministrationMemberWindowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole, DirectoryStreamMessage, DocumentExecutionTargetComponentV1, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenCheckpointV1, DocumentOpenIntentV1, DocumentOpenParentDialectV1, DocumentOpenPlanV1, DocumentOpenRendererTargetV1, DocumentScope, DocumentView, MemberSpaceViewV1, DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};
    use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
    use semio_framework_async::OperationContext;
    use std::sync::Arc;

    #[path = "../🔬️unit/🦀️.rs"]
    mod tests;

    #[path = "../../🌐️document-http/🧪️tests"]
    pub mod document_http {
        pub use semio_framework_os_directory_client::client::document_http::*;
        use semio_framework_schema_validator::OwnedJsonSchemaValidator;
        use semio_framework_os_directory_client::client::{DirectoryClient, DirectoryTransport, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, HttpMethod};
        use semio_framework_os_directory_client::DocumentScope;
        use semio_framework_async::OperationContext;

        #[path = "🦀️.rs"]
        mod tests;
    }

    #[path = "."]
    pub mod native {
        pub use semio_framework_os_directory_client::client::native::*;
        use semio_framework_os_directory_client::client::{DirectoryTransport, DirectoryWsConnection, HttpMethod, TransportError};
        use semio_framework_actor::{ActorId, PackageId};
        use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle};
        use semio_framework_os_services::{AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpBodyCancellationHandle, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead, HttpTransportStart, HttpTransportTerminalGuard, TokioHostRuntime, HTTP_BUCKET_REFILL_INTERVAL_MS};
        use std::io::Read;
        use std::net::TcpStream;
        use std::sync::{Arc, Mutex};
        use std::time::Duration;
        use tokio_tungstenite::tungstenite;
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
        use tokio_tungstenite::tungstenite::Message;

        include!("../🔬️native-runtime-identity/🦀️.rs");

        include!("../🔬️native-streaming/🦀️.rs");
    }
}

#[path = "../../../🪪️identity/🧪️tests/🔬️unit"]
pub mod identity {
    pub use semio_framework_os_directory_client::identity::*;
    use semio_framework_os_directory_client::client::{DirectoryTransport, LocalHubCredential};
    use semio_framework_async::OperationContext;
    use semio_framework_value_derive::{FromValue, ToValue};
    use std::sync::{Arc, OnceLock};

    #[path = "🦀️.rs"]
    mod tests;
}
