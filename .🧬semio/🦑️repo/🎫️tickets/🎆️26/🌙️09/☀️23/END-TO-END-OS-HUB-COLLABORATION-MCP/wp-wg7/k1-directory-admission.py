"""🐍️ WG7 K1 — one document-socket admission body on both targets (anchored, refuses on drift)."""
from pathlib import Path

P = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs")
t = P.read_text(encoding="utf-8")

def swap(old, new):
    global t
    assert t.count(old) == 1, (t.count(old), old[:120])
    t = t.replace(old, new, 1)

swap('''pub trait HubSocketGrantSource: Send + Sync {
    fn admit_document_socket(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str, timeout_ms: u64) -> Result<DocumentSocketAdmissionV1, DirectoryClientError>;
}''', '''/// 🎫 The one document-socket admission seam both document actors dial through: the open-plan
/// exchange (`POST …/open-plan`) followed by the plan-bound socket grant (`POST …/socket-grants`).
/// The semantics are one body ([`DirectoryClient`]'s `document_admission_*` legs); only the I/O shape is
/// per target: the native actor runs the blocking admission as one `Lane::Io` job, while a browser
/// isolate may not block, so there the same admission is a future on the isolate's own executor.
pub trait HubSocketGrantSource: Send + Sync {
    #[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
    fn admit_document_socket(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str, timeout_ms: u64) -> Result<DocumentSocketAdmissionV1, DirectoryClientError>;
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    fn admit_document_socket<'a>(&'a self, ctx: &'a OperationContext, space_id: &'a str, document_id: &'a str, expectation: &'a DocumentSocketExpectationV1, client_instance_id: &'a str, timeout_ms: u64) -> DocumentSocketAdmissionFuture<'a>;
}

/// ⏳ The browser admission's shape: a future polled by the isolate's executor, never blocked on.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub type DocumentSocketAdmissionFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<DocumentSocketAdmissionV1, DirectoryClientError>> + 'a>>;''')

swap('''    fn protected_post(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<WipeBytes<'static>, DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let credential = self.credential.as_ref().ok_or(DirectoryClientError::Unauthorized)?;
        let origin = credential.hub_origin().trim_end_matches('/');
        if self.base_url.trim_end_matches('/') != origin || !path.starts_with('/') || path.starts_with("//") {
            return Err(DirectoryClientError::Unauthorized);
        }
        let url = format!("{origin}{path}");
        let response = self.transport.issue_socket_grant(ctx, &url, credential.capability()?, body, timeout_ms.clamp(1, 5_000))?;
        let response_body = WipeBytes { bytes: response.body, observer: None };
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }''', '''    /// 🛡️ The exact url and bearer of one protected admission POST: a path under the credential's own
    /// origin only, so a caller can never aim the capability at a foreign host.
    fn protected_post_target(&self, ctx: &OperationContext, path: &str) -> Result<(String, &str), DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let credential = self.credential.as_ref().ok_or(DirectoryClientError::Unauthorized)?;
        let origin = credential.hub_origin().trim_end_matches('/');
        if self.base_url.trim_end_matches('/') != origin || !path.starts_with('/') || path.starts_with("//") {
            return Err(DirectoryClientError::Unauthorized);
        }
        Ok((format!("{origin}{path}"), credential.capability()?))
    }

    fn protected_post(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<WipeBytes<'static>, DirectoryClientError> {
        let (url, bearer) = self.protected_post_target(ctx, path)?;
        let response = self.transport.issue_socket_grant(ctx, &url, bearer, body, timeout_ms.clamp(1, 5_000))?;
        Self::protected_post_answer(ctx, response)
    }

    /// 🌐️ The browser twin of [`Self::protected_post`]: the same target and answer checks around the
    /// transport's own asynchronous `http` hop, because a browser isolate cannot block on a grant.
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    async fn protected_post_async(&self, ctx: &OperationContext, path: &str, body: &[u8]) -> Result<WipeBytes<'static>, DirectoryClientError> {
        let (url, bearer) = self.protected_post_target(ctx, path)?;
        let response = self.transport.http(ctx, HttpMethod::Post, &url, Some(bearer), Some(body.to_vec())).await?;
        Self::protected_post_answer(ctx, response)
    }

    /// 🧾️ Bounds and classifies one protected admission answer without ever surfacing its body.
    fn protected_post_answer(ctx: &OperationContext, response: HttpResponse) -> Result<WipeBytes<'static>, DirectoryClientError> {
        let response_body = WipeBytes { bytes: response.body, observer: None };
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }''')

swap('''    fn issue_document_socket_grant(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<DocumentSocketGrantReceiptV1, DirectoryClientError> {
        let response_body = self.protected_post(ctx, path, body, timeout_ms)?;
        let receipt: DocumentSocketGrantReceiptV1 = decode_json_bytes(&response_body.bytes).map_err(|_| DirectoryClientError::Decode("document socket grant receipt invalid".into()))?;
        if receipt.schema != "semio.hub.document-socket-grant/v1" || receipt.protocol != "semio.session.v1" || !valid_socket_actor(&receipt.actor_id) || receipt.expires_at_ms <= wall_now_ms() {
            return Err(DirectoryClientError::Decode("document socket grant receipt binding invalid".into()));
        }
        emit_document_socket_grant_probe(path);
        Ok(receipt)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: DirectoryTransport + Send + Sync> HubSocketGrantSource for DirectoryClient<T> {
    fn admit_document_socket(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str, timeout_ms: u64) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let scope = DocumentScope::new(space_id, document_id);
        let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: expectation.requested_surface_id.clone(), client_instance_id: client_instance_id.to_string() };
        if intent.validate().is_err() || expectation.artifact_schema.is_empty() {
            return Err(DirectoryClientError::Decode("document open intent invalid".into()));
        }
        let prefix = format!("/spaces/{}/documents/{}", encode_url_component(space_id), encode_url_component(document_id));
        let intent_body = crate::os_pack::json::to_json_string(&intent).into_bytes();
        let plan_body = self.protected_post(ctx, &format!("{prefix}/open-plan"), &intent_body, timeout_ms)?;
        let mut plan: DocumentOpenPlanV1 = decode_json_bytes(&plan_body.bytes).map_err(|_| DirectoryClientError::Decode("document open plan response invalid".into()))?;''', '''    /// 🎫 Admission leg one: the validated open intent, its scope and the document route it posts under.
    fn document_admission_intent(space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str) -> Result<(DocumentScope, String, Vec<u8>), DirectoryClientError> {
        let scope = DocumentScope::new(space_id, document_id);
        let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: expectation.requested_surface_id.clone(), client_instance_id: client_instance_id.to_string() };
        if intent.validate().is_err() || expectation.artifact_schema.is_empty() {
            return Err(DirectoryClientError::Decode("document open intent invalid".into()));
        }
        let prefix = format!("/spaces/{}/documents/{}", encode_url_component(space_id), encode_url_component(document_id));
        Ok((scope, prefix, crate::os_pack::json::to_json_string(&intent).into_bytes()))
    }

    /// 🧾 Admission leg two: binds the answered plan to the local expectation, derives the receipt-free
    /// authority and seals the plan-receipt exchange that buys the socket grant.
    fn document_admission_exchange(&self, ctx: &OperationContext, plan_body: &[u8], scope: &DocumentScope, expectation: &DocumentSocketExpectationV1) -> Result<(DocumentSocketAuthorityV1, WipeBytes<'static>), DirectoryClientError> {
        let mut plan: DocumentOpenPlanV1 = decode_json_bytes(plan_body).map_err(|_| DirectoryClientError::Decode("document open plan response invalid".into()))?;''')

swap('''        if plan.validate(now_ms).is_err() || plan.scope != scope || plan.artifact.schema != expectation.artifact_schema || pack_schema_hash != Some(expectation.pack_schema_hash) || !surface_matches {''', '''        if plan.validate(now_ms).is_err() || plan.scope != *scope || plan.artifact.schema != expectation.artifact_schema || pack_schema_hash != Some(expectation.pack_schema_hash) || !surface_matches {''')

swap('''        let exchange = WipeDocumentPlanSocketGrantIntent(DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: std::mem::take(&mut plan.receipt) });
        let exchange_body = WipeBytes { bytes: crate::os_pack::json::to_json_string(&exchange.0).into_bytes(), observer: None };
        let socket = self.issue_document_socket_grant(ctx, &format!("{prefix}/socket-grants"), &exchange_body.bytes, timeout_ms)?;
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if authority.expires_at_unix_ms <= u64::try_from(wall_now_ms()).map_err(|_| DirectoryClientError::Decode("document open clock invalid".into()))? {
            return Err(DirectoryClientError::Decode("document open authority expired".into()));
        }
        Ok(DocumentSocketAdmissionV1 { socket, authority })
    }
}''', '''        let exchange = WipeDocumentPlanSocketGrantIntent(DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: std::mem::take(&mut plan.receipt) });
        Ok((authority, WipeBytes { bytes: crate::os_pack::json::to_json_string(&exchange.0).into_bytes(), observer: None }))
    }

    /// 🎟 Admission leg three: the grant receipt, bound to `semio.session.v1` and a hub socket actor,
    /// joined with the authority that is still unexpired and uncancelled at this instant.
    fn document_admission_finish(ctx: &OperationContext, path: &str, grant_body: &[u8], authority: DocumentSocketAuthorityV1) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let socket: DocumentSocketGrantReceiptV1 = decode_json_bytes(grant_body).map_err(|_| DirectoryClientError::Decode("document socket grant receipt invalid".into()))?;
        if socket.schema != "semio.hub.document-socket-grant/v1" || socket.protocol != "semio.session.v1" || !valid_socket_actor(&socket.actor_id) || socket.expires_at_ms <= wall_now_ms() {
            return Err(DirectoryClientError::Decode("document socket grant receipt binding invalid".into()));
        }
        emit_document_socket_grant_probe(path);
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if authority.expires_at_unix_ms <= u64::try_from(wall_now_ms()).map_err(|_| DirectoryClientError::Decode("document open clock invalid".into()))? {
            return Err(DirectoryClientError::Decode("document open authority expired".into()));
        }
        Ok(DocumentSocketAdmissionV1 { socket, authority })
    }

    /// 🌐️ The browser admission: the same three legs as the native one, each protected POST awaited on
    /// the transport's asynchronous `http` hop.
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    async fn admit_document_socket_async(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let (scope, prefix, intent_body) = Self::document_admission_intent(space_id, document_id, expectation, client_instance_id)?;
        let plan_body = self.protected_post_async(ctx, &format!("{prefix}/open-plan"), &intent_body).await?;
        let (authority, exchange_body) = self.document_admission_exchange(ctx, &plan_body.bytes, &scope, expectation)?;
        let path = format!("{prefix}/socket-grants");
        let grant_body = self.protected_post_async(ctx, &path, &exchange_body.bytes).await?;
        Self::document_admission_finish(ctx, &path, &grant_body.bytes, authority)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: DirectoryTransport + Send + Sync> HubSocketGrantSource for DirectoryClient<T> {
    fn admit_document_socket(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str, timeout_ms: u64) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let (scope, prefix, intent_body) = Self::document_admission_intent(space_id, document_id, expectation, client_instance_id)?;
        let plan_body = self.protected_post(ctx, &format!("{prefix}/open-plan"), &intent_body, timeout_ms)?;
        let (authority, exchange_body) = self.document_admission_exchange(ctx, &plan_body.bytes, &scope, expectation)?;
        let path = format!("{prefix}/socket-grants");
        let grant_body = self.protected_post(ctx, &path, &exchange_body.bytes, timeout_ms)?;
        Self::document_admission_finish(ctx, &path, &grant_body.bytes, authority)
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl<T: DirectoryTransport + Send + Sync> HubSocketGrantSource for DirectoryClient<T> {
    fn admit_document_socket<'a>(&'a self, ctx: &'a OperationContext, space_id: &'a str, document_id: &'a str, expectation: &'a DocumentSocketExpectationV1, client_instance_id: &'a str, _timeout_ms: u64) -> DocumentSocketAdmissionFuture<'a> {
        Box::pin(self.admit_document_socket_async(ctx, space_id, document_id, expectation, client_instance_id))
    }
}''')

P.write_text(t, encoding="utf-8")
print("k1 directory admission: applied")
