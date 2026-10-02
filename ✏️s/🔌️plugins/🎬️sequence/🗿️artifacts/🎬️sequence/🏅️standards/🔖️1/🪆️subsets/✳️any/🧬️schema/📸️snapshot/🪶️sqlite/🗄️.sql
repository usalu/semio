CREATE TABLE sequence_document (
 id INTEGER PRIMARY KEY CHECK (id = 1),
 schema TEXT NOT NULL
);
CREATE TABLE sequence_content (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES sequence_document(id) CHECK (document_id = 1),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
