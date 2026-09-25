-- #region 🧲Header
-- semio/server/hub/postgres/schema.sql
-- 2026 Ueli Saluz <ueli@semio-tech.de>
-- AGPL-3.0
-- Hub persistence executed verbatim by semio-hub at startup (include_str!): persons, credentials, tokens, sessions, members, shares, operations log and periodic kit snapshots. Kit content lives only in snapshots (rs kit projection JSON) and the operations log (GraphQL mutation documents).
-- #endregion 🧲Header

-- #region 🧱Schemas
CREATE SCHEMA IF NOT EXISTS semio;
-- #endregion 🧱Schemas

-- #region 👤Identity
CREATE TABLE IF NOT EXISTS semio.person (
	id UUID PRIMARY KEY,
	name TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	color TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS semio.credential (
	person_id UUID PRIMARY KEY REFERENCES semio.person (id) ON DELETE CASCADE,
	password_hash TEXT NOT NULL,
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS semio.token (
	hash TEXT PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES semio.person (id) ON DELETE CASCADE,
	kind TEXT NOT NULL CHECK (kind IN ('login', 'agent')),
	label TEXT,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	expires_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS token_person_idx ON semio.token (person_id);
-- #endregion 👤Identity

-- #region 🏘️Sessions
CREATE TABLE IF NOT EXISTS semio.session (
	id UUID PRIMARY KEY,
	name TEXT NOT NULL,
	owner_id UUID NOT NULL REFERENCES semio.person (id) ON DELETE CASCADE,
	version BIGINT NOT NULL DEFAULT 0,
	hash TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS semio.member (
	session_id UUID NOT NULL REFERENCES semio.session (id) ON DELETE CASCADE,
	person_id UUID NOT NULL REFERENCES semio.person (id) ON DELETE CASCADE,
	role TEXT NOT NULL CHECK (role IN ('owner', 'editor', 'viewer')),
	joined_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	PRIMARY KEY (session_id, person_id)
);
CREATE INDEX IF NOT EXISTS member_person_idx ON semio.member (person_id);

CREATE TABLE IF NOT EXISTS semio.share (
	token TEXT PRIMARY KEY,
	session_id UUID NOT NULL REFERENCES semio.session (id) ON DELETE CASCADE,
	role TEXT NOT NULL CHECK (role IN ('editor', 'viewer')),
	label TEXT,
	created_by UUID NOT NULL REFERENCES semio.person (id) ON DELETE CASCADE,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS share_session_idx ON semio.share (session_id);
-- #endregion 🏘️Sessions

-- #region 📜History
CREATE TABLE IF NOT EXISTS semio.operation (
	session_id UUID NOT NULL REFERENCES semio.session (id) ON DELETE CASCADE,
	version BIGINT NOT NULL,
	operation_id TEXT NOT NULL,
	client_id TEXT NOT NULL,
	person_id UUID NOT NULL REFERENCES semio.person (id) ON DELETE CASCADE,
	participant_kind TEXT NOT NULL CHECK (participant_kind IN ('human', 'agent')),
	query TEXT NOT NULL,
	variables JSONB NOT NULL DEFAULT '{}'::jsonb,
	data JSONB NOT NULL DEFAULT 'null'::jsonb,
	hash TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	PRIMARY KEY (session_id, version),
	UNIQUE (session_id, operation_id)
);

CREATE TABLE IF NOT EXISTS semio.snapshot (
	session_id UUID NOT NULL REFERENCES semio.session (id) ON DELETE CASCADE,
	version BIGINT NOT NULL,
	hash TEXT NOT NULL,
	kit JSONB NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	PRIMARY KEY (session_id, version)
);
-- #endregion 📜History
