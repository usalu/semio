mod header {
    // 🧲Header
    // HubStorage over Neo4j (neo4rs). Users/Studios/Memberships are real nodes+relationships —
    // where graph traversal earns its keep (role lookups, VFS tree walks). The document op-log
    // stays a flat `(:Document)-[:HAS_OP]->(:Op)` fan-out, not a chained graph: an append-only log
    // gains nothing from `(:Op)-[:NEXT]->(:Op)` edges that an indexed `ORDER BY version` doesn't
    // already give a relational store, and dedupe-by-id is `MERGE` (an idempotent insert), not a
    // graph feature. Causal deps (`OpEnvelope.deps`) stay a JSON property, mirroring Postgres's
    // `jsonb` — the in-memory `OpDag` is the only thing that ever walks the causal graph.
}

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use neo4rs::{query, Graph};
use os_hub_storage::error::{StorageError, StorageResult};
use os_hub_storage::model::*;
use os_hub_storage::HubStorage;
use semio_framework_core::OpEnvelope;
use semio_framework_hash::hash_bytes;
use uuid::Uuid;

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> StorageError {
    StorageError::Backend(err.to_string())
}

fn default_snapshot() -> serde_json::Value {
    serde_json::json!({
        "schema": "s.studio/v1",
        "id": "default",
        "name": "Studio",
        "vcs": {
            "initialProjection": {
                "programs": [],
                "activeProgramId": null,
                "activeAlternativeId": null,
                "appInstances": [],
                "mediaGraph": { "schema": "s.media-graph", "nodes": [], "edges": [] }
            },
            "operations": [],
            "checkpoints": [],
            "alternatives": []
        }
    })
}

const CONSTRAINTS: &[&str] = &[
    "CREATE CONSTRAINT IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:Studio) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (d:Document) REQUIRE d.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (o:Op) REQUIRE o.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (t:ShareToken) REQUIRE t.token IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (n:Node) REQUIRE n.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:SyncSession) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (b:Blob) REQUIRE b.hash IS UNIQUE",
];

/// @emoji 🕸️ Neo4j-backed `HubStorage`.
pub struct Neo4jStorage {
    graph: Graph,
}

impl Neo4jStorage {
    /// @emoji 🔌 Connects to `uri` with `user`/`password` and bootstraps uniqueness constraints.
    pub async fn connect(uri: &str, user: &str, password: &str) -> StorageResult<Self> {
        let graph = Graph::new(uri, user, password).await.map_err(backend)?;
        for statement in CONSTRAINTS {
            graph.run(query(statement)).await.map_err(backend)?;
        }
        Ok(Self { graph })
    }

    /// @emoji 🌱 Seeds a default studio, its default document, and a `Documents/default` node.
    pub async fn seed(&self) -> StorageResult<()> {
        let mut existing = self.graph.execute(query("MATCH (s:Studio {id: 'default'}) RETURN s.id AS id")).await.map_err(backend)?;
        if existing.next().await.map_err(backend)?.is_none() {
            self.graph.run(query("CREATE (s:Studio {id: 'default', name: 'Studio', createdAt: $created_at})").param("created_at", now_ms())).await.map_err(backend)?;
        }
        self.ensure_document("default", "default").await?;
        let mut node_count = self.graph.execute(query("MATCH (n:Node) RETURN count(n) AS c")).await.map_err(backend)?;
        let count: i64 = node_count.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if count == 0 {
            let folder = self.create_node("default", None, "Documents", "folder").await?;
            self.create_node("default", Some(&folder.id), "default", "document").await?;
        }
        Ok(())
    }
}

#[async_trait]
impl HubStorage for Neo4jStorage {
    //#region Documents
    async fn ensure_document(&self, studio_id: &str, id: &str) -> StorageResult<DocumentRecord> {
        let mut result = self.graph.execute(query("MATCH (d:Document {id: $id}) RETURN d.schema AS schema, d.snapshot AS snapshot, d.version AS version").param("id", id)).await.map_err(backend)?;
        if let Some(row) = result.next().await.map_err(backend)? {
            let schema: String = row.get("schema").map_err(backend)?;
            let snapshot_json: String = row.get("snapshot").map_err(backend)?;
            let version: i64 = row.get("version").map_err(backend)?;
            let snapshot = serde_json::from_str(&snapshot_json).unwrap_or_else(|_| default_snapshot());
            return Ok(DocumentRecord { id: id.to_string(), studio_id: studio_id.to_string(), schema, snapshot, version });
        }
        let snapshot = default_snapshot();
        let schema = snapshot.get("schema").and_then(|v| v.as_str()).unwrap_or("s.studio/v1").to_string();
        self.graph
            .run(
                query(
                    "MATCH (s:Studio {id: $studio_id})
                     CREATE (d:Document {id: $id, schema: $schema, snapshot: $snapshot, version: 0})-[:IN_STUDIO]->(s)",
                )
                .param("studio_id", studio_id)
                .param("id", id)
                .param("schema", schema.clone())
                .param("snapshot", snapshot.to_string()),
            )
            .await
            .map_err(backend)?;
        Ok(DocumentRecord { id: id.to_string(), studio_id: studio_id.to_string(), schema, snapshot, version: 0 })
    }

    async fn save_document(&self, id: &str, schema: &str, snapshot: &serde_json::Value, version: i64) -> StorageResult<()> {
        self.graph
            .run(
                query(
                    "MERGE (d:Document {id: $id})
                     ON CREATE SET d.schema = $schema, d.snapshot = $snapshot, d.version = $version
                     ON MATCH SET d.schema = $schema, d.snapshot = $snapshot, d.version = $version",
                )
                .param("id", id)
                .param("schema", schema)
                .param("snapshot", snapshot.to_string())
                .param("version", version),
            )
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn insert_op(&self, document_id: &str, version: i64, envelope: &OpEnvelope) -> StorageResult<bool> {
        let payload = serde_json::to_string(envelope).unwrap_or_default();
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (d:Document {id: $document_id})
                     MERGE (o:Op {id: $id})
                     ON CREATE SET o.documentId = $document_id, o.version = $version, o.actor = $actor,
                                    o.envelope = $envelope, o.createdAt = $created_at, o.fresh = true
                     ON MATCH SET o.fresh = false
                     MERGE (d)-[:HAS_OP]->(o)
                     RETURN o.fresh AS fresh",
                )
                .param("document_id", document_id)
                .param("id", envelope.id.0.clone())
                .param("version", version)
                .param("actor", envelope.actor.0.clone())
                .param("envelope", payload)
                .param("created_at", now_ms()),
            )
            .await
            .map_err(backend)?;
        let fresh: bool = result.next().await.map_err(backend)?.and_then(|row| row.get("fresh").ok()).unwrap_or(false);
        Ok(fresh)
    }

    async fn load_ops(&self, document_id: &str) -> StorageResult<Vec<(i64, OpEnvelope)>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (:Document {id: $document_id})-[:HAS_OP]->(o:Op)
                     RETURN o.version AS version, o.envelope AS envelope ORDER BY o.version ASC",
                )
                .param("document_id", document_id),
            )
            .await
            .map_err(backend)?;
        let mut rows = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let version: i64 = row.get("version").map_err(backend)?;
            let envelope: String = row.get("envelope").map_err(backend)?;
            if let Ok(envelope) = serde_json::from_str(&envelope) {
                rows.push((version, envelope));
            }
        }
        Ok(rows)
    }
    //#endregion

    //#region Vfs
    async fn list_nodes(&self, studio_id: &str, parent: Option<&str>) -> StorageResult<Vec<NodeRecord>> {
        let cypher = match parent {
            Some(_) => "MATCH (n:Node {studioId: $studio_id})-[:PARENT]->(p:Node {id: $parent_id}) RETURN n.id AS id, p.id AS parentId, n.name AS name, n.kind AS kind ORDER BY n.name",
            None => "MATCH (n:Node {studioId: $studio_id}) WHERE NOT (n)-[:PARENT]->(:Node) RETURN n.id AS id, null AS parentId, n.name AS name, n.kind AS kind ORDER BY n.name",
        };
        let mut q = query(cypher).param("studio_id", studio_id);
        if let Some(parent) = parent {
            q = q.param("parent_id", parent);
        }
        let mut result = self.graph.execute(q).await.map_err(backend)?;
        let mut rows = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            rows.push(NodeRecord { id: row.get("id").map_err(backend)?, studio_id: studio_id.to_string(), parent_id: row.get::<String>("parentId").ok(), name: row.get("name").map_err(backend)?, kind: row.get("kind").map_err(backend)? });
        }
        Ok(rows)
    }

    async fn create_node(&self, studio_id: &str, parent_id: Option<&str>, name: &str, kind: &str) -> StorageResult<NodeRecord> {
        let id = Uuid::now_v7().to_string();
        match parent_id {
            Some(parent_id) => {
                self.graph
                    .run(
                        query(
                            "MATCH (s:Studio {id: $studio_id}), (p:Node {id: $parent_id})
                             CREATE (n:Node {id: $id, studioId: $studio_id, name: $name, kind: $kind})-[:IN_STUDIO]->(s)
                             CREATE (n)-[:PARENT]->(p)",
                        )
                        .param("studio_id", studio_id)
                        .param("parent_id", parent_id)
                        .param("id", id.clone())
                        .param("name", name)
                        .param("kind", kind),
                    )
                    .await
                    .map_err(backend)?;
            }
            None => {
                self.graph
                    .run(
                        query(
                            "MATCH (s:Studio {id: $studio_id})
                             CREATE (n:Node {id: $id, studioId: $studio_id, name: $name, kind: $kind})-[:IN_STUDIO]->(s)",
                        )
                        .param("studio_id", studio_id)
                        .param("id", id.clone())
                        .param("name", name)
                        .param("kind", kind),
                    )
                    .await
                    .map_err(backend)?;
            }
        }
        Ok(NodeRecord { id, studio_id: studio_id.to_string(), parent_id: parent_id.map(str::to_string), name: name.to_string(), kind: kind.to_string() })
    }
    //#endregion

    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> StorageResult<String> {
        let token = Uuid::now_v7().to_string();
        self.graph
            .run(
                query(
                    "MATCH (d:Document {id: $document_id})
                     CREATE (t:ShareToken {token: $token, createdAt: $created_at})-[:FOR_DOCUMENT]->(d)",
                )
                .param("document_id", document_id)
                .param("token", token.clone())
                .param("created_at", now_ms()),
            )
            .await
            .map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> StorageResult<bool> {
        let mut count_result = self.graph.execute(query("MATCH (:ShareToken)-[:FOR_DOCUMENT]->(:Document {id: $document_id}) RETURN count(*) AS c").param("document_id", document_id)).await.map_err(backend)?;
        let has_tokens: i64 = count_result.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if has_tokens == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let mut valid_result =
                    self.graph.execute(query("MATCH (t:ShareToken {token: $token})-[:FOR_DOCUMENT]->(:Document {id: $document_id}) RETURN count(*) AS c").param("token", token).param("document_id", document_id)).await.map_err(backend)?;
                let valid: i64 = valid_result.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
                Ok(valid > 0)
            }
        }
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> StorageResult<UserRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        self.graph
            .run(
                query(
                    "CREATE (u:User {id: $id, email: $email, displayName: $display_name, passwordHash: $password_hash,
                                      ssoSubject: $sso_subject, ssoProvider: $sso_provider, createdAt: $created_at})",
                )
                .param("id", id.clone())
                .param("email", email)
                .param("display_name", display_name)
                .param("password_hash", password_hash.unwrap_or_default())
                .param("sso_subject", sso_subject.unwrap_or_default())
                .param("sso_provider", sso_provider.unwrap_or_default())
                .param("created_at", created_at),
            )
            .await
            .map_err(backend)?;
        Ok(UserRecord {
            id,
            email: email.to_string(),
            display_name: display_name.to_string(),
            password_hash: password_hash.map(str::to_string),
            sso_subject: sso_subject.map(str::to_string),
            sso_provider: sso_provider.map(str::to_string),
            created_at,
        })
    }

    async fn get_user_by_email(&self, email: &str) -> StorageResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {email: $email}) RETURN u AS u").param("email", email)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> StorageResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {ssoProvider: $provider, ssoSubject: $subject}) RETURN u AS u").param("provider", provider).param("subject", subject)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> StorageResult<Vec<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User) RETURN u AS u ORDER BY u.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut users = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            users.push(user_from_node(&row)?);
        }
        Ok(users)
    }
    //#endregion

    //#region Studios
    async fn create_studio(&self, name: &str, owner_user_id: &str) -> StorageResult<StudioRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        self.graph
            .run(
                query(
                    "MATCH (u:User {id: $owner_user_id})
                     CREATE (s:Studio {id: $id, name: $name, ownerUserId: $owner_user_id, createdAt: $created_at})
                     CREATE (u)-[:MEMBER_OF {role: 'owner', createdAt: $created_at}]->(s)",
                )
                .param("id", id.clone())
                .param("name", name)
                .param("owner_user_id", owner_user_id)
                .param("created_at", created_at),
            )
            .await
            .map_err(backend)?;
        Ok(StudioRecord { id, name: name.to_string(), owner_user_id: owner_user_id.to_string(), created_at })
    }

    async fn list_studios_for_user(&self, user_id: &str) -> StorageResult<Vec<(StudioRecord, StudioRole)>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(s:Studio) RETURN s AS s, m.role AS role ORDER BY s.createdAt").param("user_id", user_id)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let studio = studio_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = StudioRole::parse(&role) {
                studios.push((studio, role));
            }
        }
        Ok(studios)
    }

    async fn list_studios(&self, limit: i64, offset: i64) -> StorageResult<Vec<StudioRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Studio) RETURN s AS s ORDER BY s.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            studios.push(studio_from_node(&row)?);
        }
        Ok(studios)
    }

    async fn list_documents_for_studio(&self, studio_id: &str) -> StorageResult<Vec<DocumentRecord>> {
        let mut result =
            self.graph.execute(query("MATCH (d:Document)-[:IN_STUDIO]->(:Studio {id: $studio_id}) RETURN d.id AS id, d.schema AS schema, d.snapshot AS snapshot, d.version AS version").param("studio_id", studio_id)).await.map_err(backend)?;
        let mut documents = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let snapshot_json: String = row.get("snapshot").map_err(backend)?;
            documents.push(DocumentRecord {
                id: row.get("id").map_err(backend)?,
                studio_id: studio_id.to_string(),
                schema: row.get("schema").map_err(backend)?,
                snapshot: serde_json::from_str(&snapshot_json).unwrap_or_else(|_| default_snapshot()),
                version: row.get("version").map_err(backend)?,
            });
        }
        Ok(documents)
    }

    async fn upsert_membership(&self, studio_id: &str, user_id: &str, role: StudioRole) -> StorageResult<()> {
        self.graph
            .run(
                query(
                    "MATCH (u:User {id: $user_id}), (s:Studio {id: $studio_id})
                     MERGE (u)-[m:MEMBER_OF]->(s)
                     ON CREATE SET m.role = $role, m.createdAt = $created_at
                     ON MATCH SET m.role = $role",
                )
                .param("user_id", user_id)
                .param("studio_id", studio_id)
                .param("role", role.as_str())
                .param("created_at", now_ms()),
            )
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn remove_membership(&self, studio_id: &str, user_id: &str) -> StorageResult<()> {
        self.graph.run(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Studio {id: $studio_id}) DELETE m").param("user_id", user_id).param("studio_id", studio_id)).await.map_err(backend)?;
        Ok(())
    }

    async fn get_role(&self, studio_id: &str, user_id: &str) -> StorageResult<Option<StudioRole>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Studio {id: $studio_id}) RETURN m.role AS role").param("user_id", user_id).param("studio_id", studio_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let role: String = row.get("role").map_err(backend)?;
                Ok(StudioRole::parse(&role))
            }
            None => Ok(None),
        }
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> StorageResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.graph
            .run(
                query(
                    "MATCH (u:User {id: $user_id})
                     CREATE (a:AuthSession {id: $id, createdAt: $created_at, expiresAt: $expires_at, ssoProvider: $sso_provider})-[:BELONGS_TO]->(u)",
                )
                .param("user_id", user_id)
                .param("id", id.clone())
                .param("created_at", created_at)
                .param("expires_at", expires_at)
                .param("sso_provider", sso_provider.unwrap_or_default()),
            )
            .await
            .map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> StorageResult<Option<AuthSessionRecord>> {
        let mut result = self
            .graph
            .execute(query("MATCH (a:AuthSession {id: $id})-[:BELONGS_TO]->(u:User) RETURN a.id AS id, u.id AS userId, a.createdAt AS createdAt, a.expiresAt AS expiresAt, a.ssoProvider AS ssoProvider").param("id", id))
            .await
            .map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(AuthSessionRecord {
                id: row.get("id").map_err(backend)?,
                user_id: row.get("userId").map_err(backend)?,
                created_at: row.get("createdAt").map_err(backend)?,
                expires_at: row.get("expiresAt").map_err(backend)?,
                sso_provider: row.get::<String>("ssoProvider").ok().filter(|s| !s.is_empty()),
            })),
            None => Ok(None),
        }
    }

    async fn revoke_auth_session(&self, id: &str) -> StorageResult<()> {
        self.graph.run(query("MATCH (a:AuthSession {id: $id}) DETACH DELETE a").param("id", id)).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(&self, document_id: &str, user_id: Option<&str>, studio_role: Option<StudioRole>, client_label: &str) -> StorageResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = studio_role.map(|r| r.as_str().to_string()).unwrap_or_default();
        match user_id {
            Some(user_id) => {
                self.graph
                    .run(
                        query(
                            "MATCH (d:Document {id: $document_id}), (u:User {id: $user_id})
                             CREATE (s:SyncSession {id: $id, clientLabel: $client_label, studioRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:FOR_DOCUMENT]->(d)
                             CREATE (s)-[:AS_USER]->(u)",
                        )
                        .param("document_id", document_id)
                        .param("user_id", user_id)
                        .param("id", id.clone())
                        .param("client_label", client_label)
                        .param("role", role_str.clone())
                        .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
            None => {
                self.graph
                    .run(
                        query(
                            "MATCH (d:Document {id: $document_id})
                             CREATE (s:SyncSession {id: $id, clientLabel: $client_label, studioRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:FOR_DOCUMENT]->(d)",
                        )
                        .param("document_id", document_id)
                        .param("id", id.clone())
                        .param("client_label", client_label)
                        .param("role", role_str.clone())
                        .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
        }
        Ok(SyncSessionRecord { id, document_id: document_id.to_string(), user_id: user_id.map(str::to_string), studio_role, client_label: client_label.to_string(), connected_at, disconnected_at: None })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> StorageResult<()> {
        self.graph.run(query("MATCH (s:SyncSession {id: $id}) SET s.disconnectedAt = $disconnected_at").param("id", sync_session_id).param("disconnected_at", now_ms())).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> StorageResult<Vec<SyncSessionRecord>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (s:SyncSession)-[:FOR_DOCUMENT]->(:Document {id: $document_id})
                     OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                     RETURN s.id AS id, u.id AS userId, s.studioRole AS role, s.clientLabel AS clientLabel,
                            s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                     ORDER BY s.connectedAt DESC",
                )
                .param("document_id", document_id),
            )
            .await
            .map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").unwrap_or_default();
            sessions.push(SyncSessionRecord {
                id: row.get("id").map_err(backend)?,
                document_id: document_id.to_string(),
                user_id: row.get::<String>("userId").ok(),
                studio_role: StudioRole::parse(&role),
                client_label: row.get("clientLabel").map_err(backend)?,
                connected_at: row.get("connectedAt").map_err(backend)?,
                disconnected_at: row.get::<i64>("disconnectedAt").ok(),
            });
        }
        Ok(sessions)
    }
    //#endregion

    //#region Blobs
    /// @emoji 🔡 Neo4j properties have no first-class byte-array type in this driver's ergonomic
    /// param API, so bytes are base64-encoded into a string property — the same "structured payload
    /// as a string" convention this backend already uses for `Document.snapshot`/`Op.envelope` (see
    /// `header`). Dedupe is `MERGE`-free here (an explicit existence check) so a re-put never pays
    /// the cost of re-encoding/re-writing the (potentially large) property.
    async fn put_blob(&self, bytes: &[u8], media_type: &str) -> StorageResult<BlobRecord> {
        let hash = hash_bytes(bytes);
        let mut existing = self.graph.execute(query("MATCH (b:Blob {hash: $hash}) RETURN b.hash AS hash").param("hash", hash.clone())).await.map_err(backend)?;
        if existing.next().await.map_err(backend)?.is_none() {
            self.graph
                .run(query("CREATE (b:Blob {hash: $hash, mediaType: $media_type, size: $size, bytes: $bytes})").param("hash", hash.clone()).param("media_type", media_type).param("size", bytes.len() as i64).param("bytes", BASE64.encode(bytes)))
                .await
                .map_err(backend)?;
        }
        Ok(BlobRecord { hash, media_type: media_type.to_string(), size: bytes.len() as i64 })
    }

    async fn get_blob(&self, hash: &str) -> StorageResult<Option<Vec<u8>>> {
        let mut result = self.graph.execute(query("MATCH (b:Blob {hash: $hash}) RETURN b.bytes AS bytes").param("hash", hash)).await.map_err(backend)?;
        if let Some(row) = result.next().await.map_err(backend)? {
            let encoded: String = row.get("bytes").map_err(backend)?;
            let decoded = BASE64.decode(encoded).map_err(backend)?;
            return Ok(Some(decoded));
        }
        Ok(None)
    }

    async fn has_blob(&self, hash: &str) -> StorageResult<bool> {
        let mut result = self.graph.execute(query("MATCH (b:Blob {hash: $hash}) RETURN count(b) AS c").param("hash", hash)).await.map_err(backend)?;
        let count: i64 = result.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        Ok(count > 0)
    }
    //#endregion
}

fn user_from_node(row: &neo4rs::Row) -> StorageResult<UserRecord> {
    let node: neo4rs::Node = row.get("u").map_err(backend)?;
    Ok(UserRecord {
        id: node.get("id").map_err(backend)?,
        email: node.get("email").map_err(backend)?,
        display_name: node.get("displayName").map_err(backend)?,
        password_hash: node.get::<String>("passwordHash").ok().filter(|s| !s.is_empty()),
        sso_subject: node.get::<String>("ssoSubject").ok().filter(|s| !s.is_empty()),
        sso_provider: node.get::<String>("ssoProvider").ok().filter(|s| !s.is_empty()),
        created_at: node.get("createdAt").map_err(backend)?,
    })
}

fn studio_from_node(row: &neo4rs::Row) -> StorageResult<StudioRecord> {
    let node: neo4rs::Node = row.get("s").map_err(backend)?;
    Ok(StudioRecord { id: node.get("id").map_err(backend)?, name: node.get("name").map_err(backend)?, owner_user_id: node.get("ownerUserId").map_err(backend)?, created_at: node.get("createdAt").map_err(backend)? })
}

//#region 🔖Tests
#[cfg(test)]
mod tests {
    // 🔬 Neo4j has no in-memory test mode; integration tests run against a live/testcontainers
    // instance per the verification plan (HP-3) — not exercised in unit-test CI without a running
    // Neo4j, unlike the sqlite backend's `:memory:` tests.
}
//#endregion 🔖Tests
