-- #region 🔖Header
-- [🧰repo📚postgres📐schema](repo://p/i/repo/b/l/postgres/f/schema.sql)
-- 2025 Ueli Saluz <ueli@semio-tech.com>
-- AGPL-3.0
-- Canonical PostgreSQL schema for repo persistence. All durable state lives here.

-- Specs:
-- - Use timestamptz everywhere.
-- - Use text for IDs (ticket IDs are path-based like "2026/04/01/slug").
-- - Use jsonb for flexible payload storage.
-- - Mirror current SQLite schema semantics but use PostgreSQL features.
-- - Include pg-boss schema via its own init.
-- #endregion 🔖Header

-- #region 🔖Extensions
-- [🧰repo📚postgres📐schema🔖extensions](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Extensions)
-- Required PostgreSQL extensions.
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
-- #endregion 🔖Extensions

-- #region 🔖Developers
-- [🧰repo📚postgres📐schema🔖developers](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Developers)
-- Identity and auth tables for trusted developer access control.

CREATE TABLE IF NOT EXISTS developers (
    id              TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    email           TEXT UNIQUE NOT NULL,
    github_login    TEXT UNIQUE,
    display_name    TEXT NOT NULL DEFAULT '',
    trusted         BOOLEAN NOT NULL DEFAULT false,
    active          BOOLEAN NOT NULL DEFAULT true,
    role            TEXT NOT NULL DEFAULT 'developer' CHECK (role IN ('developer', 'admin', 'owner')),
    discord_user_id TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at      TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS developer_api_keys (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    developer_id TEXT NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
    key_hash     TEXT NOT NULL,
    label        TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_used_at TIMESTAMPTZ,
    revoked_at   TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_developer_api_keys_hash ON developer_api_keys(key_hash) WHERE revoked_at IS NULL;

CREATE TABLE IF NOT EXISTS audit_log (
    id         TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    actor_id   TEXT REFERENCES developers(id),
    action     TEXT NOT NULL,
    target     TEXT,
    detail     JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_audit_log_created ON audit_log(created_at DESC);
-- #endregion 🔖Developers

-- #region 🔖Repos
-- [🧰repo📚postgres📐schema🔖repos](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Repos)
-- Repository registration table.

CREATE TABLE IF NOT EXISTS repos (
    id         TEXT PRIMARY KEY,
    name       TEXT NOT NULL DEFAULT '',
    path       TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- #endregion 🔖Repos

-- #region 🔖Tickets
-- [🧰repo📚postgres📐schema🔖tickets](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Tickets)
-- Ticket lifecycle tables for tracked work items.

CREATE TABLE IF NOT EXISTS tickets (
    id           TEXT PRIMARY KEY,
    status       TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'closed')),
    title        TEXT NOT NULL DEFAULT '',
    prompt       TEXT NOT NULL DEFAULT '',
    summary      TEXT NOT NULL DEFAULT '',
    llm          TEXT NOT NULL DEFAULT '',
    client       TEXT NOT NULL DEFAULT '',
    author       TEXT NOT NULL DEFAULT '',
    github_issue TEXT NOT NULL DEFAULT '',
    goal         TEXT NOT NULL DEFAULT '',
    parent       TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at    TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_tickets_status_created ON tickets(status, created_at DESC);

CREATE TABLE IF NOT EXISTS ticket_files (
    id        TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    UNIQUE(ticket_id, file_path)
);

CREATE INDEX IF NOT EXISTS idx_ticket_files_ticket ON ticket_files(ticket_id);
-- #endregion 🔖Tickets

-- #region 🔖Scopes
-- [🧰repo📚postgres📐schema🔖scopes](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Scopes)
-- Code scope indexing and claim tracking.

CREATE TABLE IF NOT EXISTS scopes (
    id              TEXT PRIMARY KEY,
    kind            TEXT NOT NULL CHECK (kind IN ('file', 'section', 'definition')),
    file_path       TEXT NOT NULL,
    section_path    TEXT NOT NULL DEFAULT '',
    definition_name TEXT NOT NULL DEFAULT '',
    start_line      INT NOT NULL DEFAULT 0,
    end_line        INT NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS ticket_claims (
    ticket_id     TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    scope_id      TEXT NOT NULL REFERENCES scopes(id) ON DELETE CASCADE,
    claim_kind    TEXT NOT NULL DEFAULT 'touched',
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (ticket_id, scope_id)
);
-- #endregion 🔖Scopes

-- #region 🔖Warnings
-- [🧰repo📚postgres📐schema🔖warnings](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Warnings)
-- Warning and breach detection tables.

CREATE TABLE IF NOT EXISTS warnings (
    id              TEXT PRIMARY KEY,
    kind            TEXT NOT NULL,
    severity        TEXT NOT NULL DEFAULT 'info',
    message         TEXT NOT NULL DEFAULT '',
    ticket_id       TEXT NOT NULL DEFAULT '',
    scope_id        TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    acknowledged_at TIMESTAMPTZ,
    ack_by          TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_warnings_ticket ON warnings(ticket_id, created_at DESC);

CREATE TABLE IF NOT EXISTS breaches (
    id          TEXT PRIMARY KEY,
    kind        TEXT NOT NULL,
    priority    TEXT NOT NULL DEFAULT 'low',
    scope_id    TEXT NOT NULL DEFAULT '',
    file_path   TEXT NOT NULL DEFAULT '',
    line        INT,
    col         INT,
    summary     TEXT NOT NULL DEFAULT '',
    excerpt     TEXT NOT NULL DEFAULT '',
    autofixable BOOLEAN NOT NULL DEFAULT false,
    detected_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ticket_id   TEXT NOT NULL DEFAULT '',
    resolved_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_breaches_ticket ON breaches(ticket_id, detected_at DESC);
-- #endregion 🔖Warnings

-- #region 🔖Events
-- [🧰repo📚postgres📐schema🔖events](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Events)
-- Event log for all system and user actions.

CREATE TABLE IF NOT EXISTS events (
    id           TEXT PRIMARY KEY,
    kind         TEXT NOT NULL,
    source       TEXT NOT NULL DEFAULT '',
    payload_json JSONB NOT NULL DEFAULT '{}',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_events_created ON events(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_events_kind_created ON events(kind, created_at DESC);
-- #endregion 🔖Events

-- #region 🔖ContributorWork
-- [🧰repo📚postgres📐schema🔖contributorwork](repo://p/i/repo/b/l/postgres/f/schema.sql/s/ContributorWork)
-- Contributor work tracking for conflict detection.

CREATE TABLE IF NOT EXISTS contributor_work (
    github  TEXT NOT NULL,
    kind    TEXT NOT NULL,
    item_id TEXT NOT NULL,
    PRIMARY KEY (github, kind, item_id)
);
-- #endregion 🔖ContributorWork

-- #region 🔖Goals
-- [🧰repo📚postgres📐schema🔖goals](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Goals)
-- Goal tracking tables.

CREATE TABLE IF NOT EXISTS goals (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'closed')),
    parent      TEXT,
    due_date    DATE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at   TIMESTAMPTZ
);
-- #endregion 🔖Goals

-- #region 🔖Artifacts
-- [🧰repo📚postgres📐schema🔖artifacts](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Artifacts)
-- Uploaded ticket folder artifacts.

CREATE TABLE IF NOT EXISTS artifacts (
    id          TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    ticket_id   TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    file_name   TEXT NOT NULL,
    file_size   BIGINT NOT NULL DEFAULT 0,
    checksum    TEXT NOT NULL DEFAULT '',
    mime_kind   TEXT NOT NULL DEFAULT 'application/gzip',
    blob_path   TEXT NOT NULL DEFAULT '',
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_artifacts_ticket ON artifacts(ticket_id);
-- #endregion 🔖Artifacts

-- #region 🔖Discord
-- [🧰repo📚postgres📐schema🔖discord](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Discord)
-- Discord delivery tracking.

CREATE TABLE IF NOT EXISTS discord_channels (
    id           TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    event_kind   TEXT NOT NULL,
    channel_id   TEXT NOT NULL,
    webhook_url  TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS discord_deliveries (
    id          TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    event_id    TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    channel_id  TEXT NOT NULL DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed', 'dead')),
    attempts    INT NOT NULL DEFAULT 0,
    last_error  TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at     TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_discord_deliveries_status ON discord_deliveries(status) WHERE status = 'pending';
-- #endregion 🔖Discord

-- #region 🔖Migration
-- [🧰repo📚postgres📐schema🔖migration](repo://p/i/repo/b/l/postgres/f/schema.sql/s/Migration)
-- Migration ledger for tracking history imports.

CREATE TABLE IF NOT EXISTS migration_runs (
    id         TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    source     TEXT NOT NULL,
    status     TEXT NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'completed', 'failed')),
    rows_ok    INT NOT NULL DEFAULT 0,
    rows_fail  INT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at   TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS migration_failures (
    id          TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    run_id      TEXT NOT NULL REFERENCES migration_runs(id) ON DELETE CASCADE,
    source_path TEXT NOT NULL DEFAULT '',
    reason      TEXT NOT NULL DEFAULT '',
    raw_data    JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- #endregion 🔖Migration