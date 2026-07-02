CREATE TABLE IF NOT EXISTS node (
    id UUID PRIMARY KEY,
    parent_id UUID REFERENCES node(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('folder', 'document'))
);

CREATE TABLE IF NOT EXISTS document (
    id TEXT PRIMARY KEY,
    node_id UUID REFERENCES node(id) ON DELETE SET NULL,
    schema TEXT NOT NULL,
    snapshot JSONB NOT NULL,
    version BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS document_op (
    id UUID PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES document(id) ON DELETE CASCADE,
    version BIGINT NOT NULL,
    author TEXT,
    change JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS session (
    id UUID PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES document(id) ON DELETE CASCADE,
    client_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS share_token (
    token TEXT PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES session(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('owner', 'share'))
);
