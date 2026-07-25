-- #region 🔖Header
-- 2026 Ueli Saluz
-- AGPL-3.0
-- os-hub Postgres schema — idempotent bootstrap (CREATE ... IF NOT EXISTS), no migration framework
-- (greenfield: there are no users yet, so schema changes are edited in place, not migrated).
-- #endregion 🔖Header

CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_studio (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_studio_membership (
    studio_id TEXT NOT NULL REFERENCES hub_studio(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('owner', 'member', 'viewer')),
    created_at BIGINT NOT NULL,
    PRIMARY KEY (studio_id, user_id)
);

CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    sso_provider TEXT
);

CREATE TABLE IF NOT EXISTS hub_sync_session (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    studio_role TEXT,
    client_label TEXT NOT NULL,
    connected_at BIGINT NOT NULL,
    disconnected_at BIGINT
);

CREATE TABLE IF NOT EXISTS hub_node (
    id TEXT PRIMARY KEY,
    studio_id TEXT NOT NULL REFERENCES hub_studio(id) ON DELETE CASCADE,
    parent_id TEXT REFERENCES hub_node(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_document (
    id TEXT PRIMARY KEY,
    studio_id TEXT NOT NULL REFERENCES hub_studio(id) ON DELETE CASCADE,
    schema TEXT NOT NULL,
    snapshot JSONB NOT NULL,
    version BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS hub_document_operation (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES hub_document(id) ON DELETE CASCADE,
    version BIGINT NOT NULL,
    actor TEXT,
    envelope JSONB NOT NULL,
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_share_token (
    token TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES hub_document(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_blob (
    hash TEXT PRIMARY KEY,
    media_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    bytes BYTEA NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_studio_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_node_studio_parent ON hub_node (studio_id, parent_id);
CREATE INDEX IF NOT EXISTS idx_op_document_version ON hub_document_operation (document_id, version);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
