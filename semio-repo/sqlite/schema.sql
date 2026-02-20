-- #region 🔖Header

-- sql/sqlite/repo/schema.sql

-- 2025 Ueli Saluz <ueli@semio-tech.com>

-- #region 🔖License

-- This program is free software: you can redistribute it and/or modify
-- it under the terms of the GNU Affero General Public License as
-- published by the Free Software Foundation, either version 3 of the
-- License, or (at your option) any later version.

-- This program is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU Affero General Public License for more details.

-- You should have received a copy of the GNU Affero General Public License
-- along with this program.  If not, see <https://www.gnu.org/licenses/>.


-- #endregion 🔖License

-- #region 🔖Specs
-- #endregion 🔖Specs

-- #endregion 🔖Header

-- #region 🔖Tables

-- #region 🔖Repo
CREATE TABLE IF NOT EXISTS contributor (
    id TEXT PRIMARY KEY,
    github TEXT NOT NULL UNIQUE,
    name TEXT,
    avatar TEXT,
);
CREATE TABLE IF NOT EXISTS commit (
    sha TEXT PRIMARY KEY,
    message TEXT NOT NULL,
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    date TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS repo (
    id TEXT PRIMARY KEY,
    sha TEXT NOT NULL REFERENCES commit(sha) ON DELETE CASCADE,
);
CREATE TABLE IF NOT EXISTS folder (
    id TEXT PRIMARY KEY,
    repo_id TEXT NOT NULL REFERENCES repo(id) ON DELETE CASCADE,
    parent_id TEXT REFERENCES folder(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    bundle_id TEXT REFERENCES bundle(id) ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS file (
    id TEXT PRIMARY KEY,
    parent_folder_id TEXT NOT NULL REFERENCES folder(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
);
CREATE TABLE IF NOT EXISTS bundle (
    id TEXT PRIMARY KEY,
    kind TEXT,
    folder_id TEXT NOT NULL REFERENCES folder(id) ON DELETE CASCADE
);

-- #endregion 🔖Bundle


-- #region 🔖File

CREATE TABLE IF NOT EXISTS file (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    uri TEXT NOT NULL,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    folder_id TEXT REFERENCES folder(id) ON DELETE SET NULL,
    bundle_id TEXT REFERENCES bundle(id) ON DELETE SET NULL,
    lines INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_file_folder ON file(folder_id);
CREATE INDEX IF NOT EXISTS idx_file_bundle ON file(bundle_id);
CREATE INDEX IF NOT EXISTS idx_file_extension ON file(extension);

-- #endregion 🔖File

-- #region 🔖Section

CREATE TABLE IF NOT EXISTS section (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    file_id TEXT NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    parent_id TEXT REFERENCES section(id) ON DELETE CASCADE,
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    start_column INTEGER NOT NULL DEFAULT 0,
    end_column INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_section_file ON section(file_id);
CREATE INDEX IF NOT EXISTS idx_section_parent ON section(parent_id);

-- #endregion 🔖Section

-- #region 🔖Definition

CREATE TABLE IF NOT EXISTS definition (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('function', 'class', 'variable', 'interface', 'type', 'enum', 'method', 'property')),
    file_id TEXT NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    section_id TEXT REFERENCES section(id) ON DELETE SET NULL,
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    start_column INTEGER NOT NULL DEFAULT 0,
    end_column INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_definition_file ON definition(file_id);
CREATE INDEX IF NOT EXISTS idx_definition_section ON definition(section_id);
CREATE INDEX IF NOT EXISTS idx_definition_kind ON definition(kind);

-- #endregion 🔖Definition

-- #region 🔖Contributor

CREATE TABLE IF NOT EXISTS contributor (
    id TEXT PRIMARY KEY,
    github TEXT NOT NULL UNIQUE,
    name TEXT,
    avatar_url TEXT,
    avatar_round_url TEXT,
    github_icon_url TEXT
);

CREATE TABLE IF NOT EXISTS contributor_email (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    PRIMARY KEY (contributor_id, email)
);

CREATE TABLE IF NOT EXISTS contributor_link (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    PRIMARY KEY (contributor_id, name)
);

-- #endregion 🔖Contributor

-- #region 🔖Commit

CREATE TABLE IF NOT EXISTS commit_record (
    id TEXT PRIMARY KEY,
    sha TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    author_id TEXT REFERENCES contributor(id) ON DELETE SET NULL,
    date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commit_bundle (
    commit_id TEXT NOT NULL REFERENCES commit_record(id) ON DELETE CASCADE,
    bundle_id TEXT NOT NULL REFERENCES bundle(id) ON DELETE CASCADE,
    PRIMARY KEY (commit_id, bundle_id)
);

CREATE TABLE IF NOT EXISTS commit_file (
    commit_id TEXT NOT NULL REFERENCES commit_record(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    PRIMARY KEY (commit_id, file_id)
);

-- #endregion 🔖Commit

-- #region 🔖Ticket

CREATE TABLE IF NOT EXISTS ticket (
    id TEXT PRIMARY KEY,
    year INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL,
    slug TEXT NOT NULL,
    title TEXT,
    path TEXT NOT NULL,
    uri TEXT NOT NULL,
    prompt TEXT NOT NULL,
    summary TEXT,
    status TEXT NOT NULL CHECK (status IN ('open', 'closed', 'finished')),
    author_id TEXT REFERENCES contributor(id) ON DELETE SET NULL,
    model TEXT,
    llm TEXT,
    ui TEXT,
    commit_sha TEXT,
    created_at TEXT NOT NULL,
    finished_at TEXT,
    UNIQUE (year, month, day, slug)
);

CREATE INDEX IF NOT EXISTS idx_ticket_status ON ticket(status);
CREATE INDEX IF NOT EXISTS idx_ticket_date ON ticket(year, month, day);
CREATE INDEX IF NOT EXISTS idx_ticket_author ON ticket(author_id);

-- #endregion 🔖Ticket

-- #region 🔖Ticket Checkpoint

CREATE TABLE IF NOT EXISTS ticket_checkpoint (
    id TEXT PRIMARY KEY,
    ticket_id TEXT NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    sequence INTEGER NOT NULL,
    kind TEXT NOT NULL,
    prompt TEXT NOT NULL,
    model TEXT,
    author_id TEXT REFERENCES contributor(id) ON DELETE SET NULL,
    commit_sha TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_checkpoint_ticket ON ticket_checkpoint(ticket_id);

CREATE TABLE IF NOT EXISTS checkpoint_file (
    checkpoint_id TEXT NOT NULL REFERENCES ticket_checkpoint(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    PRIMARY KEY (checkpoint_id, file_path)
);

CREATE TABLE IF NOT EXISTS checkpoint_section (
    id TEXT PRIMARY KEY,
    checkpoint_id TEXT NOT NULL REFERENCES ticket_checkpoint(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    section_name TEXT NOT NULL,
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    lines_added INTEGER NOT NULL DEFAULT 0,
    lines_removed INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_checkpoint_section_checkpoint ON checkpoint_section(checkpoint_id);

CREATE TABLE IF NOT EXISTS checkpoint_definition (
    checkpoint_section_id TEXT NOT NULL REFERENCES checkpoint_section(id) ON DELETE CASCADE,
    definition_name TEXT NOT NULL,
    PRIMARY KEY (checkpoint_section_id, definition_name)
);

-- #endregion 🔖Ticket Checkpoint

-- #region 🔖Policy

CREATE TABLE IF NOT EXISTS policy (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS policy_scope (
    policy_id TEXT NOT NULL REFERENCES policy(id) ON DELETE CASCADE,
    scope TEXT NOT NULL,
    PRIMARY KEY (policy_id, scope)
);

-- #endregion 🔖Policy

-- #region 🔖Breach Kind

CREATE TABLE IF NOT EXISTS statute (
    id TEXT PRIMARY KEY,
    policy_id TEXT NOT NULL REFERENCES policy(id) ON DELETE CASCADE,
    priority TEXT NOT NULL CHECK (priority IN ('high', 'medium', 'low')),
    autofixable INTEGER NOT NULL DEFAULT 0,
    reason TEXT NOT NULL,
    solution TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_statute_policy ON statute(policy_id);
CREATE INDEX IF NOT EXISTS idx_statute_priority ON statute(priority);

-- #endregion 🔖Breach Kind

-- #region 🔖Breach

CREATE TABLE IF NOT EXISTS breach (
    id TEXT PRIMARY KEY,
    kind_id TEXT NOT NULL REFERENCES statute(id) ON DELETE CASCADE,
    scope TEXT NOT NULL,
    file_id TEXT REFERENCES file(id) ON DELETE SET NULL,
    folder_id TEXT REFERENCES folder(id) ON DELETE SET NULL,
    line INTEGER,
    column_num INTEGER,
    excerpt TEXT,
    summary TEXT NOT NULL,
    autofix_description TEXT,
    autofix_edits TEXT
);

CREATE INDEX IF NOT EXISTS idx_statute ON breach(kind_id);
CREATE INDEX IF NOT EXISTS idx_breach_file ON breach(file_id);
CREATE INDEX IF NOT EXISTS idx_breach_folder ON breach(folder_id);

-- #endregion 🔖Breach

-- #region 🔖Contribution

CREATE TABLE IF NOT EXISTS contribution_bundle (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    bundle_id TEXT NOT NULL REFERENCES bundle(id) ON DELETE CASCADE,
    added INTEGER NOT NULL DEFAULT 0,
    updated INTEGER NOT NULL DEFAULT 0,
    removed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (contributor_id, bundle_id)
);

CREATE TABLE IF NOT EXISTS contribution_folder (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    folder_id TEXT NOT NULL REFERENCES folder(id) ON DELETE CASCADE,
    added INTEGER NOT NULL DEFAULT 0,
    updated INTEGER NOT NULL DEFAULT 0,
    removed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (contributor_id, folder_id)
);

CREATE TABLE IF NOT EXISTS contribution_file (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    lines_added INTEGER NOT NULL DEFAULT 0,
    lines_removed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (contributor_id, file_id)
);

CREATE TABLE IF NOT EXISTS contribution_section (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    section_id TEXT NOT NULL REFERENCES section(id) ON DELETE CASCADE,
    lines_added INTEGER NOT NULL DEFAULT 0,
    lines_removed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (contributor_id, section_id)
);

CREATE TABLE IF NOT EXISTS contribution_definition (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    definition_id TEXT NOT NULL REFERENCES definition(id) ON DELETE CASCADE,
    lines_added INTEGER NOT NULL DEFAULT 0,
    lines_removed INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (contributor_id, definition_id)
);

-- #endregion 🔖Contribution

-- #region 🔖Ticket Bundle

CREATE TABLE IF NOT EXISTS ticket_bundle (
    ticket_id TEXT NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    bundle_id TEXT NOT NULL REFERENCES bundle(id) ON DELETE CASCADE,
    PRIMARY KEY (ticket_id, bundle_id)
);

CREATE TABLE IF NOT EXISTS ticket_file (
    ticket_id TEXT NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    PRIMARY KEY (ticket_id, file_id)
);

-- #endregion 🔖Ticket Bundle

-- #region 🔖Contributor Bundle

CREATE TABLE IF NOT EXISTS contributor_bundle (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    bundle_id TEXT NOT NULL REFERENCES bundle(id) ON DELETE CASCADE,
    PRIMARY KEY (contributor_id, bundle_id)
);

CREATE TABLE IF NOT EXISTS contributor_file (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    PRIMARY KEY (contributor_id, file_id)
);

CREATE TABLE IF NOT EXISTS contributor_ticket (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    ticket_id TEXT NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    PRIMARY KEY (contributor_id, ticket_id)
);

CREATE TABLE IF NOT EXISTS contributor_commit (
    contributor_id TEXT NOT NULL REFERENCES contributor(id) ON DELETE CASCADE,
    commit_id TEXT NOT NULL REFERENCES commit_record(id) ON DELETE CASCADE,
    PRIMARY KEY (contributor_id, commit_id)
);

-- #endregion 🔖Contributor Bundle

-- #endregion 🔖Tables

-- #region 🔖Views

CREATE VIEW IF NOT EXISTS repo_metrics AS
SELECT
    (SELECT COUNT(*) FROM bundle) AS bundles,
    (SELECT COUNT(*) FROM folder) AS folders,
    (SELECT COUNT(*) FROM file) AS files,
    (SELECT COUNT(*) FROM section) AS sections,
    (SELECT COUNT(*) FROM definition) AS definitions,
    (SELECT COALESCE(SUM(lines), 0) FROM file) AS lines,
    (SELECT COUNT(*) FROM contributor) AS contributors,
    (SELECT COUNT(*) FROM ticket) AS tickets,
    (SELECT COUNT(*) FROM breach) AS breachs;

CREATE VIEW IF NOT EXISTS bundle_metrics_view AS
SELECT
    b.id AS bundle_id,
    (SELECT COUNT(*) FROM folder f WHERE f.bundle_id = b.id) AS folders,
    (SELECT COUNT(*) FROM file f WHERE f.bundle_id = b.id) AS files,
    (SELECT COUNT(*) FROM section s JOIN file f ON s.file_id = f.id WHERE f.bundle_id = b.id) AS sections,
    (SELECT COUNT(*) FROM definition d JOIN file f ON d.file_id = f.id WHERE f.bundle_id = b.id) AS definitions,
    (SELECT COALESCE(SUM(f.lines), 0) FROM file f WHERE f.bundle_id = b.id) AS lines,
    (SELECT COUNT(*) FROM breach v JOIN file f ON v.file_id = f.id WHERE f.bundle_id = b.id) AS breachs
FROM bundle b;

CREATE VIEW IF NOT EXISTS folder_metrics_view AS
SELECT
    fo.id AS folder_id,
    (SELECT COUNT(*) FROM file f WHERE f.folder_id = fo.id) AS files,
    (SELECT COALESCE(SUM(f.lines), 0) FROM file f WHERE f.folder_id = fo.id) AS lines,
    (SELECT COUNT(*) FROM breach v WHERE v.folder_id = fo.id) AS breachs
FROM folder fo;

CREATE VIEW IF NOT EXISTS file_metrics_view AS
SELECT
    f.id AS file_id,
    (SELECT COUNT(*) FROM section s WHERE s.file_id = f.id) AS sections,
    (SELECT COUNT(*) FROM definition d WHERE d.file_id = f.id) AS definitions,
    f.lines AS lines
FROM file f;

CREATE VIEW IF NOT EXISTS section_metrics_view AS
SELECT
    s.id AS section_id,
    (SELECT COUNT(*) FROM definition d WHERE d.section_id = s.id) AS definitions,
    (s.end_line - s.start_line + 1) AS lines,
    (SELECT COUNT(*) FROM breach v WHERE v.scope LIKE '%#' || s.name) AS breachs
FROM section s;

CREATE VIEW IF NOT EXISTS breach_priority_counts AS
SELECT
    (SELECT COUNT(*) FROM breach v JOIN statute vk ON v.kind_id = vk.id WHERE vk.priority = 'high') AS high,
    (SELECT COUNT(*) FROM breach v JOIN statute vk ON v.kind_id = vk.id WHERE vk.priority = 'medium') AS medium,
    (SELECT COUNT(*) FROM breach v JOIN statute vk ON v.kind_id = vk.id WHERE vk.priority = 'low') AS low;

CREATE VIEW IF NOT EXISTS contributor_metrics_view AS
SELECT
    c.id AS contributor_id,
    (SELECT COUNT(*) FROM contributor_commit cc WHERE cc.contributor_id = c.id) AS commits,
    (SELECT COUNT(*) FROM contributor_ticket ct WHERE ct.contributor_id = c.id) AS tickets,
    (SELECT COUNT(*) FROM contributor_bundle cb WHERE cb.contributor_id = c.id) AS bundles,
    (SELECT COUNT(*) FROM contribution_folder cf WHERE cf.contributor_id = c.id) AS folders,
    (SELECT COUNT(*) FROM contribution_file cfi WHERE cfi.contributor_id = c.id) AS files,
    (SELECT COUNT(*) FROM contribution_section cs WHERE cs.contributor_id = c.id) AS sections,
    (SELECT COUNT(*) FROM contribution_definition cd WHERE cd.contributor_id = c.id) AS definitions,
    (SELECT COALESCE(SUM(cfi.lines_added + cfi.lines_removed), 0) FROM contribution_file cfi WHERE cfi.contributor_id = c.id) AS lines
FROM contributor c;

CREATE VIEW IF NOT EXISTS ticket_metrics_view AS
SELECT
    t.id AS ticket_id,
    (SELECT COUNT(*) FROM ticket_checkpoint tc WHERE tc.ticket_id = t.id) AS checkpoints,
    (SELECT COUNT(DISTINCT cf.file_path) FROM ticket_checkpoint tc JOIN checkpoint_file cf ON cf.checkpoint_id = tc.id WHERE tc.ticket_id = t.id) AS files,
    (SELECT COUNT(*) FROM ticket_checkpoint tc JOIN checkpoint_section cs ON cs.checkpoint_id = tc.id WHERE tc.ticket_id = t.id) AS sections,
    (SELECT COUNT(*) FROM ticket_checkpoint tc JOIN checkpoint_section cs ON cs.checkpoint_id = tc.id JOIN checkpoint_definition cd ON cd.checkpoint_section_id = cs.id WHERE tc.ticket_id = t.id) AS definitions,
    (SELECT COALESCE(SUM(cs.lines_added), 0) FROM ticket_checkpoint tc JOIN checkpoint_section cs ON cs.checkpoint_id = tc.id WHERE tc.ticket_id = t.id) AS lines_added,
    (SELECT COALESCE(SUM(cs.lines_removed), 0) FROM ticket_checkpoint tc JOIN checkpoint_section cs ON cs.checkpoint_id = tc.id WHERE tc.ticket_id = t.id) AS lines_removed
FROM ticket t;

-- #endregion 🔖Views
