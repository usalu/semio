CREATE TABLE flow_document (id INTEGER PRIMARY KEY, schema_id TEXT NOT NULL);
CREATE TABLE flow_content (id INTEGER PRIMARY KEY REFERENCES flow_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
