CREATE TABLE vcs_document (
 id INTEGER PRIMARY KEY CHECK (id = 1),
 schema TEXT NOT NULL,
 title TEXT NOT NULL,
 counter INTEGER NOT NULL,
 notes TEXT NOT NULL,
 status TEXT NOT NULL
);
CREATE TABLE vcs_tag (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES vcs_document(id) CHECK (document_id = 1),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 value TEXT NOT NULL
);

