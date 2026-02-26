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

-- #region 🔖Requirements
-- #endregion 🔖Requirements

-- #endregion 🔖Header
CREATE TABLE IF NOT EXISTS repo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    summary TEXT,
    checkpoint TEXT NOT NULL -- e.g. 
);
CREATE TABLE IF NOT EXISTS folder (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_folder_id INTEGER REFERENCES folder(id) ON DELETE CASCADE,
    kind INTEGER NOT NULL DEFAULT 0, -- 0:🗃️organizational 1:🛅required
    name TEXT NOT NULL,
    summary TEXT,
    UNIQUE (parent_folder_id, name)
);
CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_id INTEGER NOT NULL REFERENCES folder(id) ON DELETE CASCADE,
    kind INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL UNIQUE,
    summary TEXT
);
CREATE TABLE IF NOT EXISTS bundle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    folder_id INTEGER NOT NULL REFERENCES folder(id) ON DELETE CASCADE,
    kind INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    summary TEXT,
    UNIQUE (project_id, kind, name)
);
CREATE TABLE IF NOT EXISTS file (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_folder_id INTEGER REFERENCES folder(id) ON DELETE CASCADE,
    kind INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    summary TEXT,
    UNIQUE (parent_folder_id, kind, name)
);
CREATE TABLE IF NOT EXISTS section (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL REFERENCES file(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    summary TEXT,
    UNIQUE (file_id, name)
);
CREATE TABLE IF NOT EXISTS definition (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id INTEGER NOT NULL REFERENCES section(id) ON DELETE CASCADE,
    kind INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    summary TEXT,
    code TEXT,
    UNIQUE (section_id, kind, name)
);