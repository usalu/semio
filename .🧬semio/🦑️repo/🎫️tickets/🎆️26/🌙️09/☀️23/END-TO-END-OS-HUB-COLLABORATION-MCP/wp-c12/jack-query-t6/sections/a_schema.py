# 🧬️ Section A of `c12-jack-query-document-patch.py`: `query` joins JackArtifact, JackSnapshot, JackDiff and every codec of them.
SNAP = SCHEMA + "📸️snapshot/"
ART = SCHEMA
DIFF = SCHEMA + "🔺️diff/"
WIRE = SCHEMA + "🛜️wire-runtime/🦀️.rs"

QUERY_DOC = "    /// 🔎️ The document's Jack query — the query editor's text, document content like the graph it runs against.\n"

# ── Rust: snapshot ──
edit(SNAP + "🦀️.rs", """    #[state(artifact)]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Snapshot""", """    #[state(artifact)]
    pub root_node_id: Option<String>,
""" + QUERY_DOC + """    #[state(artifact)]
    pub query: String,
}
//#endregion 🔖️Snapshot""", "snapshot struct")
edit(SNAP + "🦀️.rs", "Vec::with_capacity(7);", "Vec::with_capacity(8);", "snapshot to_value capacity")
edit(SNAP + "🦀️.rs", """            entries.push(("rootNodeId".to_string(), dsl::ToValue::to_value(root_node_id)));
        }
        dsl::DslValue::object(entries)""", """            entries.push(("rootNodeId".to_string(), dsl::ToValue::to_value(root_node_id)));
        }
        entries.push(("query".to_string(), dsl::ToValue::to_value(&self.query)));
        dsl::DslValue::object(entries)""", "snapshot to_value query")
edit(SNAP + "🦀️.rs", """            root_node_id: match get("rootNodeId") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => None,
            },
        })""", """            root_node_id: match get("rootNodeId") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => None,
            },
            query: dsl::FromValue::from_value(get("query").ok_or_else(|| dsl::ValueError::new("missing field `query`"))?)?,
        })""", "snapshot from_value query")
edit(SNAP + "🦀️.rs", "content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()), root_node_id: None }\n    }\n}\n\n//#region 🌉️ExternalCodecBridge",
     "content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()), root_node_id: None, query: crate::TRINITY_JACK_DEFAULT_QUERY.into() }\n    }\n}\n\n//#region 🌉️ExternalCodecBridge", "snapshot default query")

# ── Rust: artifact ──
edit(ART + "🦀️.rs", """    #[state(artifact)]
    pub root_node_id: Option<String>,
}
//#endregion 🔖️Artifact""", """    #[state(artifact)]
    pub root_node_id: Option<String>,
""" + QUERY_DOC + """    #[state(artifact)]
    pub query: String,
}
//#endregion 🔖️Artifact""", "artifact struct")
edit(ART + "🦀️.rs", """            ("rootNodeId".to_string(), dsl::ToValue::to_value(&self.root_node_id)),
        ])""", """            ("rootNodeId".to_string(), dsl::ToValue::to_value(&self.root_node_id)),
            ("query".to_string(), dsl::ToValue::to_value(&self.query)),
        ])""", "artifact to_value query")
edit(ART + "🦀️.rs", """            root_node_id: dsl::FromValue::from_value(field("rootNodeId")?)?,
        })""", """            root_node_id: dsl::FromValue::from_value(field("rootNodeId")?)?,
            query: dsl::FromValue::from_value(field("query")?)?,
        })""", "artifact from_value query")
edit(ART + "🦀️.rs", "content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()), root_node_id: None }\n    }\n}\n\nimpl JackArtifact {",
     "content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()), root_node_id: None, query: crate::TRINITY_JACK_DEFAULT_QUERY.into() }\n    }\n}\n\nimpl JackArtifact {", "artifact default query")
edit(ART + "🦀️.rs", """            root_node_id: self.root_node_id.clone(),
        }
    }""", """            root_node_id: self.root_node_id.clone(),
            query: self.query.clone(),
        }
    }""", "artifact to_snapshot query")
edit(ART + "🦀️.rs", "root_node_id: snapshot.root_node_id }", "root_node_id: snapshot.root_node_id, query: snapshot.query }", "artifact from_snapshot query")
edit(ART + "🦀️.rs", "        self.root_node_id = snapshot.root_node_id;\n", "        self.root_node_id = snapshot.root_node_id;\n        self.query = snapshot.query;\n", "artifact set_snapshot query")

# ── Rust: diff ──
edit(DIFF + "🦀️.rs", """    #[state(artifact)]
    pub root_node_id: Option<Option<String>>,
}""", """    #[state(artifact)]
    pub root_node_id: Option<Option<String>>,
    #[state(artifact)]
    pub query: Option<String>,
}""", "diff struct")

# ── JSON Schema / GraphQL / proto / TypeScript: artifact + snapshot ──
for base, label in ((ART, "artifact"), (SNAP, "snapshot")):
    edit(base + "🔣️.json", '"required": ["schema", "name", "manifest", "camera", "content"],', '"required": ["schema", "name", "manifest", "camera", "content", "query"],', f"{label} json required")
    edit(base + "🔗️.graphql", "  rootNodeId: String @state(class: ARTIFACT)\n}", "  rootNodeId: String @state(class: ARTIFACT)\n  query: String! @state(class: ARTIFACT)\n}", f"{label} graphql")
    edit(base + "🛰️.proto", "  optional string root_node_id = 7;\n}", "  optional string root_node_id = 7;\n  string query = 8;\n}", f"{label} proto")
    edit(base + "🟦️.ts", "  /** @state artifact */ rootNodeId?: string;\n}", "  /** @state artifact */ rootNodeId?: string;\n  /** @state artifact */ query: string;\n}", f"{label} ts interface")
edit(ART + "🔣️.json", """    "rootNodeId": { "type": "string", "x-semio-state": "artifact" }
  },
  "$defs\"""", """    "rootNodeId": { "type": "string", "x-semio-state": "artifact" },
    "query": { "type": "string", "x-semio-state": "artifact" }
  },
  "$defs\"""", "artifact json query")
edit(SNAP + "🔣️.json", """    "rootNodeId": { "type": "string", "x-semio-state": "artifact" }
  }
}""", """    "rootNodeId": { "type": "string", "x-semio-state": "artifact" },
    "query": { "type": "string", "x-semio-state": "artifact" }
  }
}""", "snapshot json query")
edit(ART + "🟦️.ts", 'exact(row, ["schema", "name", "manifest", "camera", "content"], ["manifestId", "rootNodeId"], at);',
     'exact(row, ["schema", "name", "manifest", "camera", "content", "query"], ["manifestId", "rootNodeId"], at);', "artifact ts exact")
edit(ART + "🟦️.ts", """    rootNodeId: row.rootNodeId === undefined ? undefined : string(row.rootNodeId, `${at}.rootNodeId`),
  };""", """    rootNodeId: row.rootNodeId === undefined ? undefined : string(row.rootNodeId, `${at}.rootNodeId`),
    query: string(row.query, `${at}.query`),
  };""", "artifact ts parse")

# ── JSON Schema / GraphQL / proto / TypeScript: diff ──
edit(DIFF + "🔣️.json", '"required": ["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId"],',
     '"required": ["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId", "query"],', "diff json required")
edit(DIFF + "🔣️.json", """    "rootNodeId": { "type": ["string", "null"], "x-semio-state": "artifact" }
  }
}""", """    "rootNodeId": { "type": ["string", "null"], "x-semio-state": "artifact" },
    "query": { "type": ["string", "null"], "x-semio-state": "artifact" }
  }
}""", "diff json query")
edit(DIFF + "🔗️.graphql", "  rootNodeId: String @state(class: ARTIFACT)\n}", "  rootNodeId: String @state(class: ARTIFACT)\n  query: String @state(class: ARTIFACT)\n}", "diff graphql")
edit(DIFF + "🛰️.proto", "  optional string root_node_id = 7;\n}", "  optional string root_node_id = 7;\n  optional string query = 8;\n}", "diff proto")
edit(DIFF + "🟦️.ts", "  /** @state artifact */ rootNodeId: string | null;\n}", "  /** @state artifact */ rootNodeId: string | null;\n  /** @state artifact */ query: string | null;\n}", "diff ts interface")
edit(DIFF + "🟦️.ts", 'const keys = ["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId"];',
     'const keys = ["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId", "query"];', "diff ts keys")
edit(DIFF + "🟦️.ts", """    rootNodeId: nullableString(row.rootNodeId, `${at}.rootNodeId`),
  };""", """    rootNodeId: nullableString(row.rootNodeId, `${at}.rootNodeId`),
    query: nullableString(row.query, `${at}.query`),
  };""", "diff ts parse")

# ── text/pack record ──
edit(SNAP + "📝️text/🦀️.rs", """    root_node_id: Option<String>,
}

impl JackPackRecord {""", """    root_node_id: Option<String>,
    query: String,
}

impl JackPackRecord {""", "pack record field")
edit(SNAP + "📝️text/🦀️.rs", """            root_node_id: snapshot.root_node_id.clone(),
        }
    }""", """            root_node_id: snapshot.root_node_id.clone(),
            query: snapshot.query.clone(),
        }
    }""", "pack record from_snapshot")
edit(SNAP + "📝️text/🦀️.rs", "camera, content, root_node_id: self.root_node_id };", "camera, content, root_node_id: self.root_node_id, query: self.query };", "pack record into_snapshot")

# ── crate root: default query, JSON import/export, executor graph ──
edit(ROOT, "pub const TRINITY_GRAPH_SCHEMA: &str = JackSnapshot::SCHEMA;\n", "pub const TRINITY_GRAPH_SCHEMA: &str = JackSnapshot::SCHEMA;\n\n/// 🔎️ The Jack query a new document opens with — `JackSnapshot::query` is document content, undoable and shared like the graph.\npub const TRINITY_JACK_DEFAULT_QUERY: &str = \"" + DEFAULT_QUERY + "\";\n", "root default query")
edit(ROOT, """            "rootNodeId": self.root_node_id,
        });""", """            "rootNodeId": self.root_node_id,
            "query": self.query,
        });""", "root to_json query")
edit(ROOT, """        let mut snapshot = Self::with_content(schema, name, manifest_id, manifest, camera, JackWorkingScene { nodes: nodes, edges: edges }, root_node_id);""",
     """        let query = value.get("query").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let mut snapshot = Self { query, ..Self::with_content(schema, name, manifest_id, manifest, camera, JackWorkingScene { nodes: nodes, edges: edges }, root_node_id) };""", "root from_json query")
edit(ROOT, """    /// 🏗️ Transfers one working scene into the snapshot's exact composed content owner.
    pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, scene: JackWorkingScene, root_node_id: Option<String>) -> Self {
        Self { schema, name, manifest_id, manifest, camera, content: jack_content_child_with_owner(scene.nodes, scene.edges), root_node_id }""",
     """    /// 🏗️ Transfers one working scene into the snapshot's exact composed content owner. The query starts empty: a document
    /// constructor names its own (`Self { query, ..Self::with_content(..) }`).
    pub fn with_content(schema: String, name: String, manifest_id: Option<String>, manifest: Manifest, camera: Camera, scene: JackWorkingScene, root_node_id: Option<String>) -> Self {
        Self { schema, name, manifest_id, manifest, camera, content: jack_content_child_with_owner(scene.nodes, scene.edges), root_node_id, query: String::new() }""", "root with_content query")
edit(ROOT, """    pub root_node_id: Option<String>,
}

impl Graph {""", """    pub root_node_id: Option<String>,
    pub query: String,
}

impl Graph {""", "root graph field")
edit(ROOT, "camera: snapshot.camera, nodes, edges, root_node_id: snapshot.root_node_id })", "camera: snapshot.camera, nodes, edges, root_node_id: snapshot.root_node_id, query: snapshot.query })", "root graph from_snapshot")
edit(ROOT, """        JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), self.name.clone(), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: self.nodes.values().cloned().collect(), edges: self.edges.values().cloned().collect() }, self.root_node_id.clone())""",
     """        JackSnapshot { query: self.query.clone(), ..JackSnapshot::with_content(JackSnapshot::SCHEMA.to_string(), self.name.clone(), self.manifest_id.clone(), self.manifest.clone(), self.camera.clone(), JackWorkingScene { nodes: self.nodes.values().cloned().collect(), edges: self.edges.values().cloned().collect() }, self.root_node_id.clone()) }""", "root graph to_snapshot")
edit(ROOT, """    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "trinity".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: Vec::new(), edges: Vec::new() }, None)""",
     """    JackSnapshot { query: TRINITY_JACK_DEFAULT_QUERY.into(), ..JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "trinity".into(), Some("nakagin".into()), Manifest::nakagin_default(), Camera::default(), JackWorkingScene { nodes: Vec::new(), edges: Vec::new() }, None) }""", "root empty fixture query")

# ── executor result graphs and the store initializer ──
edit(SCHEMA + "🧮️executor/🪜️execution/🦀️.rs", """                        root_node_id: if self.root_selected { graph.root_node_id.take() } else { None },
                    };""", """                        root_node_id: if self.root_selected { graph.root_node_id.take() } else { None },
                        query: String::new(),
                    };""", "executor result fixture query")
edit(WIRE, "content, root_node_id: None })),", "content, root_node_id: None, query: String::new() })),", "wire placeholder query")
edit(WIRE, """                    self.phase = 13;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                _ => {""", """                    self.phase = 13;
                    store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
                }
                13 => Self::phased_string_step(&mut value.query, &mut self.phase, 14, maximum_items, maximum_bytes),
                _ => {""", "wire retirement query")
edit(WIRE, """            12 => {
                target.root_node_id = source.root_node_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.root_node_id.as_deref().map_or(0, str::len)
            }
""", """            12 => {
                target.root_node_id = source.root_node_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.root_node_id.as_deref().map_or(0, str::len)
            }
            13 => {
                target.query = Self::clone_string(&source.query, maximum_bytes)?;
                source.query.len()
            }
""", "wire clone query")
edit(WIRE, "                12 => digest.observe(source.root_node_id.as_deref().unwrap_or_default().as_bytes()),\n",
     "                12 => digest.observe(source.root_node_id.as_deref().unwrap_or_default().as_bytes()),\n                13 => digest.observe(source.query.as_bytes()),\n", "wire digest query")

# ── diff application: `query` applies and absorbs like every other sparse slot ──
DIFF_APPLY = SCHEMA + "🔺️diff/📝️text/🦀️.rs"
replace_every(DIFF_APPLY, """            if let Some(value) = &self.root_node_id {
                next.root_node_id = value.clone();
            }
            next
""", """            if let Some(value) = &self.root_node_id {
                next.root_node_id = value.clone();
            }
            if let Some(value) = &self.query {
                next.query = value.clone();
            }
            next
""", "diff apply query")
edit(DIFF_APPLY, """        take!(root_node_id);
    }""", """        take!(root_node_id);
        take!(query);
    }""", "diff absorb query")
