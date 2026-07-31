//! @emoji 🏛️ `architect` — compose query language: Jack (mathematical_graph_dsl) is the parsing
//! front end, this crate owns only the GraphQL planner/executor that lowers Jack's AST into
//! `Transport` calls. See `plans/every-dsl-must-be-crispy-shell.md` (Wave 2 / P9) for the
//! repo-wide unified-DSL-syntax rationale: Architect used to carry its own hand-rolled `nom`
//! lexer/parser/AST; that is gone now, replaced by `mathematical_graph_dsl::parse`.

#![allow(clippy::too_many_lines, reason = "planner/wasm_api export match arms enumerate every AST/Step variant inline; splitting them up would scatter one concept across helper fns for no clarity gain")]

pub use api::{compile, parse, plan, run};
pub use errors::ArchitectError;
pub use executor::Executor;
pub use executor::QueryResult;
pub use planner::OpPlan;
#[cfg(not(target_arch = "wasm32"))]
pub use transport::ComposeTransport;
pub use transport::{MemoryTransport, OpKind, Transport, TransportError};

#[cfg(target_arch = "wasm32")]
pub use wasm_api::{architect_compile, architect_run};

/// 🌉️ `mathematical_graph_manifest::PropertyValue` (Jack literal values) <-> `serde_json::Value`
/// (GraphQL/row values) — `PropertyValue` is `#[serde(untagged)]`, so this is exactly its JSON
/// shape; shared by `schema::call_variables`, `executor`'s `WHERE` comparisons, and `wasm_api`.
fn property_value_to_json(value: &mathematical_graph_manifest::PropertyValue) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

//#region 🔖️Errors
mod errors {
    use thiserror::Error;

    /// @emoji ⚠️ Architect parse/plan/execute failure.
    #[derive(Debug, Error)]
    pub enum ArchitectError {
        #[error("parse: {0}")]
        Parse(String),
        #[error("plan: {0}")]
        Plan(String),
        #[error("execute: {0}")]
        Execute(String),
        #[error("transport: {0}")]
        Transport(#[from] super::transport::TransportError),
    }
}
//#endregion 🔖️Errors

//#region 🔖️Schema
mod schema {
    use mathematical_graph_dsl as jack;
    use mathematical_graph_manifest::PropertyValue;

    /// @emoji 🏷️ GraphQL object label in architect patterns.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Label {
        Kit,
        Design,
        Piece,
        Blueprint,
        Type,
        Port,
        Connector,
        Connection,
        Side,
        Session,
        Store,
        Quality,
        Author,
        Tag,
        Concept,
        Prop,
        Attribute,
        Representation,
        Layer,
        Group,
    }

    impl Label {
        pub fn parse(s: &str) -> Option<Self> {
            Some(match s {
                "Kit" => Self::Kit,
                "Design" => Self::Design,
                "Piece" => Self::Piece,
                "Blueprint" => Self::Blueprint,
                "Type" => Self::Type,
                "Port" => Self::Port,
                "Connector" => Self::Connector,
                "Connection" => Self::Connection,
                "Side" => Self::Side,
                "Session" => Self::Session,
                "Store" => Self::Store,
                "Quality" => Self::Quality,
                "Author" => Self::Author,
                "Tag" => Self::Tag,
                "Concept" => Self::Concept,
                "Prop" => Self::Prop,
                "Attribute" => Self::Attribute,
                "Representation" => Self::Representation,
                "Layer" => Self::Layer,
                "Group" => Self::Group,
                _ => return None,
            })
        }

        pub fn gql_name(self) -> &'static str {
            match self {
                Self::Kit => "Kit",
                Self::Design => "Design",
                Self::Piece => "Piece",
                Self::Blueprint => "Blueprint",
                Self::Type => "Type",
                Self::Port => "Port",
                Self::Connector => "Connector",
                Self::Connection => "Connection",
                Self::Side => "Side",
                Self::Session => "Session",
                Self::Store => "Store",
                Self::Quality => "Quality",
                Self::Author => "Author",
                Self::Tag => "Tag",
                Self::Concept => "Concept",
                Self::Prop => "Prop",
                Self::Attribute => "Attribute",
                Self::Representation => "Representation",
                Self::Layer => "Layer",
                Self::Group => "Group",
            }
        }
    }

    /// @emoji 🔗️ Architect relationship predicate. `Parent`/`Child` replace the old boolean
    /// `{parent: true/false}` edge-property disambiguation — Jack's pattern grammar carries no
    /// property-map literal, so the two `Connection -> Side` edges are told apart by predicate
    /// keyword instead (`[:PARENT]` / `[:CHILD]`).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Predicate {
        Has,
        Is,
        References,
        Owns,
        Parent,
        Child,
    }

    impl Predicate {
        pub fn parse(s: &str) -> Option<Self> {
            Some(match s.to_ascii_uppercase().as_str() {
                "HAS" => Self::Has,
                "IS" => Self::Is,
                "REFERENCES" => Self::References,
                "OWNS" => Self::Owns,
                "PARENT" => Self::Parent,
                "CHILD" => Self::Child,
                _ => return None,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Cardinality {
        One,
        Many,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct EdgeDef {
        pub from: Label,
        pub pred: Predicate,
        pub to: Label,
        pub field: &'static str,
        pub cardinality: Cardinality,
    }

    pub const EDGES: &[EdgeDef] = &[
        EdgeDef { from: Label::Piece, pred: Predicate::Has, to: Label::Blueprint, field: "blueprint", cardinality: Cardinality::One },
        EdgeDef { from: Label::Blueprint, pred: Predicate::Is, to: Label::Type, field: "__typename", cardinality: Cardinality::One },
        EdgeDef { from: Label::Blueprint, pred: Predicate::Is, to: Label::Design, field: "__typename", cardinality: Cardinality::One },
        EdgeDef { from: Label::Type, pred: Predicate::Has, to: Label::Connector, field: "hasConnectors", cardinality: Cardinality::Many },
        EdgeDef { from: Label::Type, pred: Predicate::Has, to: Label::Port, field: "hasPorts", cardinality: Cardinality::Many },
        EdgeDef { from: Label::Connector, pred: Predicate::Is, to: Label::Port, field: "port", cardinality: Cardinality::One },
        EdgeDef { from: Label::Side, pred: Predicate::References, to: Label::Connector, field: "connector", cardinality: Cardinality::One },
        EdgeDef { from: Label::Connection, pred: Predicate::Parent, to: Label::Side, field: "parent", cardinality: Cardinality::One },
        EdgeDef { from: Label::Connection, pred: Predicate::Child, to: Label::Side, field: "child", cardinality: Cardinality::One },
        EdgeDef { from: Label::Design, pred: Predicate::Has, to: Label::Connection, field: "hasConnections", cardinality: Cardinality::Many },
        EdgeDef { from: Label::Design, pred: Predicate::Has, to: Label::Piece, field: "hasPieces", cardinality: Cardinality::Many },
        EdgeDef { from: Label::Kit, pred: Predicate::Has, to: Label::Design, field: "hasDesigns", cardinality: Cardinality::Many },
        EdgeDef { from: Label::Kit, pred: Predicate::Has, to: Label::Type, field: "hasTypes", cardinality: Cardinality::Many },
    ];

    pub fn resolve_edge(from: Label, pred: Predicate, to: Label) -> Result<EdgeDef, super::ArchitectError> {
        let matches: Vec<EdgeDef> = EDGES.iter().copied().filter(|e| e.from == from && e.pred == pred && e.to == to).collect();
        match matches.len() {
            0 => Err(super::ArchitectError::Plan(format!("no edge {from:?}-{pred:?}->{to:?}"))),
            1 => Ok(matches[0]),
            _ => Err(super::ArchitectError::Plan(format!("ambiguous edge {from:?}-{pred:?}->{to:?}"))),
        }
    }

    pub fn node_label(node: &jack::PatternNode) -> Result<Label, super::ArchitectError> {
        Label::parse(&node.kind).ok_or_else(|| super::ArchitectError::Plan(format!("unknown label {}", node.kind)))
    }

    pub fn rel_predicate(kind: Option<&str>) -> Result<Predicate, super::ArchitectError> {
        let t = kind.ok_or_else(|| super::ArchitectError::Plan("relationship requires a predicate".to_string()))?;
        Predicate::parse(t).ok_or_else(|| super::ArchitectError::Plan(format!("unknown predicate {t}")))
    }

    pub fn entity_scalar_fields(label: Label) -> &'static [&'static str] {
        match label {
            Label::Design => &["id", "hash", "name"],
            Label::Type => &["id", "hash", "name"],
            Label::Piece => &["id", "hash", "name"],
            Label::Port => &["id", "hash", "label", "code"],
            Label::Connector => &["id", "hash", "name"],
            Label::Connection => &["id", "hash", "name"],
            Label::Side => &["id", "hash"],
            Label::Kit => &["id", "hash", "name"],
            _ => &["id", "hash"],
        }
    }

    /// @emoji 📞️ Static `CALL` target (mutation or subscription). Jack's `CALL <ident>(...)`
    /// grammar takes a single, un-dotted identifier (no `session.end`-style path), so targets are
    /// named with flat camelCase idents instead of the old dotted `path: &[&str]`.
    #[derive(Debug, Clone, Copy)]
    pub struct CallTarget {
        pub name: &'static str,
        pub kind: super::transport::OpKind,
        pub gql: &'static str,
    }

    pub const CALL_TARGETS: &[CallTarget] = &[
        CallTarget { name: "sessionStart", kind: super::transport::OpKind::Mutation, gql: "mutation ArchitectCall($input: String) { session { start } }" },
        CallTarget { name: "sessionEnd", kind: super::transport::OpKind::Mutation, gql: "mutation ArchitectCall { session { end { ok errors { message } } } }" },
        CallTarget { name: "subscriptionSession", kind: super::transport::OpKind::Subscription, gql: "subscription ArchitectSub { session { id hash } }" },
        CallTarget { name: "subscriptionOperation", kind: super::transport::OpKind::Subscription, gql: "subscription ArchitectSub { operation { id hash } }" },
        CallTarget {
            name: "installProjection",
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall($storeId: ID!, $json: String!) { session { store(id: $storeId) { installProjection(json: $json) { ok errors { message } } } } }",
        },
        CallTarget {
            name: "startNewChange",
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange { ok errors { message } } } } } }",
        },
        CallTarget { name: "saveKit", kind: super::transport::OpKind::Mutation, gql: "mutation ArchitectCall($storeId: ID!) { session { store(id: $storeId) { theKit { save { ok errors { message } } } } } }" },
    ];

    pub fn resolve_call(name: &str) -> Result<CallTarget, super::ArchitectError> {
        CALL_TARGETS.iter().copied().find(|t| t.name == name).ok_or_else(|| super::ArchitectError::Plan(format!("unknown CALL target {name}")))
    }

    /// @emoji 🎛️ Maps a `CALL name(args...)`'s positional `PropertyValue` args onto the named
    /// GraphQL variables its target document expects — positional-by-convention since Jack's
    /// `CALL` grammar has no keyed/object-literal argument syntax.
    pub fn call_variables(name: &str, args: &[PropertyValue]) -> serde_json::Value {
        let mut vars = serde_json::Map::new();
        match name {
            "installProjection" => {
                if let Some(store) = args.first() {
                    vars.insert("storeId".into(), super::property_value_to_json(store));
                }
                if let Some(json) = args.get(1) {
                    vars.insert("json".into(), super::property_value_to_json(json));
                }
            }
            "startNewChange" | "saveKit" => {
                if let Some(store) = args.first() {
                    vars.insert("storeId".into(), super::property_value_to_json(store));
                }
            }
            _ => {}
        }
        serde_json::Value::Object(vars)
    }
}
//#endregion 🔖️Schema

//#region 🔖️Transport
mod transport {
    use futures_util::stream::{self, BoxStream};
    #[cfg(not(target_arch = "wasm32"))]
    use futures_util::StreamExt;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use thiserror::Error;

    /// @emoji 📡️ GraphQL operation kind for the host transport.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub enum OpKind {
        Query,
        Mutation,
        Subscription,
    }

    /// @emoji ⚠️ Transport-level failure.
    #[derive(Debug, Error)]
    pub enum TransportError {
        #[error("{0}")]
        Msg(String),
    }

    /// 🌊️ Async result of [`Transport::subscribe`]: a stream of GraphQL payloads, or a transport-level failure.
    pub type SubscribeResult = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>;

    /// @emoji 🌐️ Async GraphQL IO boundary (native + wasm).
    pub trait Transport {
        fn execute(&self, kind: OpKind, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>>;

        fn subscribe(&self, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = SubscribeResult> + '_>>;
    }

    /// @emoji 🧪️ In-memory transport for unit tests.
    pub struct MemoryTransport {
        pub responses: HashMap<String, Value>,
    }

    impl MemoryTransport {
        pub fn new(responses: HashMap<String, Value>) -> Self {
            Self { responses }
        }

        fn key(kind: OpKind, doc: &str) -> String {
            format!("{kind:?}:{doc}")
        }
    }

    impl Transport for MemoryTransport {
        fn execute(&self, kind: OpKind, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>> {
            let key = Self::key(kind, doc);
            let _ = variables;
            let out = self.responses.get(&key).or_else(|| self.responses.get(doc)).cloned().ok_or_else(|| TransportError::Msg(format!("no canned response for {key}")));
            Box::pin(async move { out })
        }

        fn subscribe(&self, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>> + '_>> {
            let key = Self::key(OpKind::Subscription, doc);
            let _ = variables;
            let item = self.responses.get(&key).or_else(|| self.responses.get(doc)).cloned().ok_or_else(|| TransportError::Msg(format!("no canned subscription for {key}")));
            Box::pin(async move { Ok(Box::pin(stream::once(async move { item })) as BoxStream<'static, Result<Value, TransportError>>) })
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub struct JsTransport {
        execute_fn: js_sys::Function,
        subscribe_fn: js_sys::Function,
    }

    #[cfg(target_arch = "wasm32")]
    impl JsTransport {
        pub fn new(execute_fn: js_sys::Function, subscribe_fn: js_sys::Function) -> Self {
            Self { execute_fn, subscribe_fn }
        }
    }

    #[cfg(target_arch = "wasm32")]
    impl Transport for JsTransport {
        fn execute(&self, kind: OpKind, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>> {
            let execute_fn = self.execute_fn.clone();
            let doc = doc.to_string();
            let kind_s = format!("{kind:?}");
            Box::pin(async move {
                let vars = serde_wasm_bindgen::to_value(&variables).map_err(|e| TransportError::Msg(e.to_string()))?;
                let promise = execute_fn.call2(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(&kind_s), &wasm_bindgen::JsValue::from_str(&doc)).map_err(|e| TransportError::Msg(format!("{e:?}")))?;
                let _ = vars;
                let val = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&promise)).await.map_err(|e| TransportError::Msg(format!("{e:?}")))?;
                serde_wasm_bindgen::from_value(val).map_err(|e| TransportError::Msg(e.to_string()))
            })
        }

        fn subscribe(&self, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>> + '_>> {
            let subscribe_fn = self.subscribe_fn.clone();
            let doc = doc.to_string();
            let vars = variables;
            Box::pin(async move {
                let _vars = vars;
                let _stream_factory = subscribe_fn.call1(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_str(&doc)).map_err(|e| TransportError::Msg(format!("{e:?}")))?;
                Err(TransportError::Msg("JsTransport subscription stream wiring is host-specific".into()))
            })
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// @emoji 🏗️ Executes planned GraphQL against a live compose [`compose::gql::AppSchema`].
    pub struct ComposeTransport {
        schema: compose::gql::AppSchema,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl ComposeTransport {
        pub fn new(schema: compose::gql::AppSchema) -> Self {
            Self { schema }
        }

        fn gql_value(data: &async_graphql::Value) -> Result<Value, TransportError> {
            data.clone().into_json().map_err(|e| TransportError::Msg(e.to_string()))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl Transport for ComposeTransport {
        fn execute(&self, kind: OpKind, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>> {
            let _ = kind;
            let doc = doc.to_string();
            Box::pin(async move {
                use async_graphql::{Request, Variables};
                let mut req = Request::new(doc);
                if !variables.is_null() {
                    req = req.variables(Variables::from_json(variables));
                }
                let res = self.schema.execute(req).await;
                if !res.errors.is_empty() {
                    return Err(TransportError::Msg(format!("{:?}", res.errors)));
                }
                let payload = Self::gql_value(&res.data)?;
                Ok(serde_json::json!({ "data": payload }))
            })
        }

        fn subscribe(&self, doc: &str, variables: Value) -> Pin<Box<dyn Future<Output = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>> + '_>> {
            let doc = doc.to_string();
            Box::pin(async move {
                use async_graphql::{Request, Variables};
                let mut req = Request::new(doc);
                if !variables.is_null() {
                    req = req.variables(Variables::from_json(variables));
                }
                let mut sub = self.schema.execute_stream(req);
                let first = sub.next().await.ok_or_else(|| TransportError::Msg("empty subscription".into()))?;
                if !first.errors.is_empty() {
                    return Err(TransportError::Msg(format!("{:?}", first.errors)));
                }
                let payload = Self::gql_value(&first.data)?;
                Ok(Box::pin(stream::once(async move { Ok(serde_json::json!({ "data": payload })) })) as BoxStream<'static, Result<Value, TransportError>>)
            })
        }
    }
}
//#endregion 🔖️Transport

//#region 🔖️Planner
mod planner {
    use super::errors::ArchitectError;
    use super::schema::{self, Label};
    use super::transport::OpKind;
    use mathematical_graph_dsl as jack;
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, BTreeSet};

    /// @emoji 🧭️ Planned execution steps for an architect query, lowered from Jack's `Query`.
    #[derive(Debug, Clone, PartialEq)]
    pub struct OpPlan {
        pub steps: Vec<Step>,
        pub return_items: Option<Vec<jack::ReturnItem>>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Step {
        GraphQl { operator: OpKind, document: String, variables: Value, bind: BindSpec },
        Join { on_var: String, key: String },
        Filter { expr: jack::Expr },
        Unwind { source_var: String, source_prop: Option<String>, alias: String },
        Project { items: Vec<jack::ReturnItem> },
        Call { operator: OpKind, document: String, variables: Value },
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BindSpec {
        pub anchor_var: String,
        pub anchor_label: String,
        pub paths: BTreeMap<String, JsonPath>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct JsonPath {
        pub segments: Vec<PathSeg>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum PathSeg {
        Field { name: String },
        ConnectionEdges,
        ConnectionNode,
    }

    struct PatternPlan {
        document: String,
        bind: BindSpec,
    }

    /// @emoji 🧭️ Lower Jack's `Query` into an `OpPlan`. Jack's `MATCH` clause carries no nested
    /// `WHERE`/multi-hop chaining of its own (each `Pattern` is at most one node-edge-node hop,
    /// and `WHERE` always shows up as its own following `Clause`), so a long relationship chain is
    /// simply written as several comma-separated single-hop patterns sharing var names — the
    /// existing shared-var `Join` detection below already handles stitching them back together.
    pub fn plan_query(q: &jack::Query) -> Result<OpPlan, ArchitectError> {
        let mut steps = Vec::new();
        let mut emitted_patterns: BTreeSet<String> = BTreeSet::new();
        let mut join_vars: BTreeSet<String> = BTreeSet::new();
        let mut return_items = None;

        for clause in &q.clauses {
            match clause {
                jack::Clause::Match(patterns) => {
                    let mut shared: BTreeMap<String, usize> = BTreeMap::new();
                    for pat in patterns {
                        *shared.entry(pat.nodes[0].var.clone()).or_default() += 1;
                        if let Some(edge) = &pat.edge {
                            *shared.entry(edge.right.var.clone()).or_default() += 1;
                        }
                    }
                    for pat in patterns {
                        let plan = plan_pattern(pat)?;
                        if emitted_patterns.insert(plan.document.clone()) {
                            steps.push(Step::GraphQl { operator: OpKind::Query, document: plan.document, variables: json!({}), bind: plan.bind });
                        }
                    }
                    for (var, count) in &shared {
                        if *count > 1 && join_vars.insert(var.clone()) {
                            steps.push(Step::Join { on_var: var.clone(), key: "id".into() });
                        }
                    }
                }
                jack::Clause::Where(expr) => steps.push(Step::Filter { expr: expr.clone() }),
                jack::Clause::With(items) => steps.push(Step::Project { items: items.clone() }),
                jack::Clause::Unwind(u) => {
                    let (source_var, source_prop) = match &u.source {
                        jack::ReturnItem::Var(v) => (v.clone(), None),
                        jack::ReturnItem::Property { var, prop } => (var.clone(), Some(prop.clone())),
                    };
                    steps.push(Step::Unwind { source_var, source_prop, alias: u.var.clone() });
                }
                jack::Clause::Call(c) => {
                    let target = schema::resolve_call(&c.name)?;
                    steps.push(Step::Call { operator: target.kind, document: target.gql.to_string(), variables: schema::call_variables(&c.name, &c.args) });
                }
                jack::Clause::Return(items) => return_items = Some(items.clone()),
                jack::Clause::Create(_) | jack::Clause::Delete(_) | jack::Clause::Set(_) | jack::Clause::Merge(_) => {
                    return Err(ArchitectError::Plan("mutating jack clauses are not supported by architect".into()));
                }
            }
        }

        Ok(OpPlan { steps, return_items })
    }

    fn plan_pattern(pat: &jack::Pattern) -> Result<PatternPlan, ArchitectError> {
        let left = &pat.nodes[0];
        let right = pat.edge.as_ref().map(|e| &e.right);
        let anchor_is_left = match right {
            None => true,
            Some(r) => selectivity(left) <= selectivity(r),
        };
        let anchor = if anchor_is_left { left } else { right.expect("right present whenever !anchor_is_left") };
        let anchor_label = schema::node_label(anchor)?;

        let (document, paths) = build_graphql_document(pat, anchor_is_left, anchor_label)?;
        let bind = BindSpec { anchor_var: anchor.var.clone(), anchor_label: anchor_label.gql_name().to_string(), paths };
        Ok(PatternPlan { document, bind })
    }

    fn selectivity(n: &jack::PatternNode) -> u8 {
        let mut score = 20u8;
        if let Some(l) = Label::parse(&n.kind) {
            if matches!(l, Label::Kit | Label::Design | Label::Type) {
                score = score.saturating_sub(8);
            } else {
                score = score.saturating_add(12);
            }
        }
        score
    }

    fn build_graphql_document(pat: &jack::Pattern, anchor_is_left: bool, anchor_label: Label) -> Result<(String, BTreeMap<String, JsonPath>), ArchitectError> {
        let mut paths: BTreeMap<String, JsonPath> = BTreeMap::new();
        let mut anchor_path = vec![
            PathSeg::Field { name: "session".into() },
            PathSeg::Field { name: "stores".into() },
            PathSeg::ConnectionEdges,
            PathSeg::ConnectionNode,
            PathSeg::Field { name: "wip".into() },
            PathSeg::Field { name: "theKit".into() },
            PathSeg::Field { name: "kit".into() },
        ];

        match anchor_label {
            Label::Design => {
                anchor_path.push(PathSeg::Field { name: "hasDesigns".into() });
                anchor_path.push(PathSeg::ConnectionEdges);
                anchor_path.push(PathSeg::ConnectionNode);
            }
            Label::Type => {
                anchor_path.push(PathSeg::Field { name: "hasTypes".into() });
                anchor_path.push(PathSeg::ConnectionEdges);
                anchor_path.push(PathSeg::ConnectionNode);
            }
            Label::Kit => {}
            _ => {
                return Err(ArchitectError::Plan(format!("anchor label {} must be Kit, Design, or Type for session root", anchor_label.gql_name())));
            }
        }

        let anchor_var = if anchor_is_left { pat.nodes[0].var.clone() } else { pat.edge.as_ref().expect("edge present whenever !anchor_is_left").right.var.clone() };
        paths.insert(anchor_var, JsonPath { segments: anchor_path.clone() });
        // 🩹️ the non-anchor side of a hop (if any) is bound to the SAME anchor row rather than its
        // own nested entity — this crate never returns fields of the "many" side of a hop directly
        // (see `build_nested_selection`'s cardinality-aware embedding below), only uses it for
        // `WHERE`/`Join` purposes, so a precise per-row extraction isn't needed here.
        if anchor_is_left {
            if let Some(edge) = &pat.edge {
                paths.entry(edge.right.var.clone()).or_insert(JsonPath { segments: anchor_path.clone() });
            }
        } else {
            paths.entry(pat.nodes[0].var.clone()).or_insert(JsonPath { segments: anchor_path.clone() });
        }

        let selection = if anchor_is_left { build_nested_selection(anchor_label, pat.edge.as_ref()) } else { build_nested_selection(anchor_label, None) };

        let mut body = String::from("query ArchitectMatch {\n  session {\n    stores {\n      edges {\n        node {\n          wip {\n            theKit {\n              kit {\n");
        match anchor_label {
            Label::Design => {
                body.push_str("                hasDesigns {\n                  edges {\n                    node {\n");
                body.push_str(&selection);
                body.push_str("                    }\n                  }\n                }\n");
            }
            Label::Type => {
                body.push_str("                hasTypes {\n                  edges {\n                    node {\n");
                body.push_str(&selection);
                body.push_str("                    }\n                  }\n                }\n");
            }
            Label::Kit => {
                body.push_str(&selection);
            }
            _ => {}
        }
        body.push_str("              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n");
        Ok((body, paths))
    }

    fn build_nested_selection(anchor_label: Label, edge: Option<&jack::PatternEdge>) -> String {
        let mut out = String::new();
        let scalars = schema::entity_scalar_fields(anchor_label).join(" ");
        out.push_str(&format!("                      {scalars}\n"));
        let Some(edge) = edge else { return out };
        let Ok(to_label) = schema::node_label(&edge.right) else { return out };
        let Ok(pred) = schema::rel_predicate(edge.kind.as_deref()) else { return out };
        let Ok(edge_def) = schema::resolve_edge(anchor_label, pred, to_label) else { return out };
        if edge_def.field == "__typename" {
            if to_label == Label::Type {
                out.push_str("                      ... on Type { id hash name connectors { edges { node { id hash name port { id hash label code } } } } }\n");
            } else if to_label == Label::Design {
                out.push_str("                      ... on Design { id hash name }\n");
            }
        } else if edge_def.cardinality == schema::Cardinality::Many {
            let child_scalars = schema::entity_scalar_fields(to_label).join(" ");
            out.push_str(&format!("                      {} {{ edges {{ node {{ {child_scalars} }} }} }}\n", edge_def.field));
        } else {
            let child_scalars = schema::entity_scalar_fields(to_label).join(" ");
            out.push_str(&format!("                      {} {{ {child_scalars} }}\n", edge_def.field));
        }
        out
    }
}
//#endregion 🔖Planner

//#region 🔖Executor
mod executor {
    use super::errors::ArchitectError;
    use super::planner::{JsonPath, OpPlan, PathSeg, Step};
    use super::transport::{OpKind, Transport};
    use futures_util::{stream, StreamExt};
    use mathematical_graph_dsl as jack;
    use mathematical_graph_manifest::PropertyValue;
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::pin::Pin;

    pub type Row = BTreeMap<String, Value>;

    /// @emoji 📊 Tabular architect result.
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct QueryResult {
        pub columns: Vec<String>,
        pub rows: Vec<Value>,
    }

    /// @emoji ⚙ Runs `OpPlan` against a `Transport`.
    pub struct Executor;

    impl Executor {
        pub async fn run(plan: &OpPlan, transport: &dyn Transport) -> Result<QueryResult, ArchitectError> {
            let mut env = BindEnv::default();
            for step in &plan.steps {
                env.apply(step, transport).await?;
            }
            Ok(env.finish(plan.return_items.as_deref()))
        }

        pub async fn run_subscription(plan: &OpPlan, transport: &dyn Transport) -> Result<Pin<Box<dyn futures_util::Stream<Item = Result<QueryResult, ArchitectError>> + Send>>, ArchitectError> {
            let has_sub = plan.steps.iter().any(|s| matches!(s, Step::Call { operator: OpKind::Subscription, .. }));
            if !has_sub {
                return Err(ArchitectError::Execute("plan has no subscription CALL".into()));
            }
            let mut env = BindEnv::default();
            for step in &plan.steps {
                match step {
                    Step::Call { operator: OpKind::Subscription, document, variables, .. } => {
                        let mut sub_stream = transport.subscribe(document, variables.clone()).await?;
                        let ret = plan.return_items.clone();
                        let first = sub_stream.next().await.ok_or_else(|| ArchitectError::Execute("empty subscription stream".into()))??;
                        env.rows.clear();
                        env.ingest_call_result(&first);
                        let once = stream::once(async move { Ok(env.finish(ret.as_deref())) });
                        return Ok(Box::pin(once));
                    }
                    other => env.apply(other, transport).await?,
                }
            }
            Err(ArchitectError::Execute("subscription step not reached".into()))
        }
    }

    #[derive(Default)]
    struct BindEnv {
        rows: Vec<Row>,
    }

    impl BindEnv {
        async fn apply(&mut self, step: &Step, transport: &dyn Transport) -> Result<(), ArchitectError> {
            match step {
                Step::GraphQl { operator, document, variables, bind } => {
                    let data = transport.execute(*operator, document, variables.clone()).await?;
                    let expanded = extract_rows(&data, bind)?;
                    if self.rows.is_empty() {
                        self.rows = expanded;
                    } else {
                        self.rows = cartesian_merge(&self.rows, &expanded);
                    }
                }
                Step::Join { on_var, key } => {
                    self.rows = join_rows(&self.rows, on_var, key);
                }
                Step::Filter { expr } => {
                    self.rows.retain(|row| eval_bool(expr, row));
                }
                Step::Unwind { source_var, source_prop, alias } => {
                    let mut next = Vec::new();
                    for row in &self.rows {
                        let Some(base) = row.get(source_var) else { continue };
                        let v = match source_prop {
                            Some(p) => base.get(p).cloned().unwrap_or(Value::Null),
                            None => base.clone(),
                        };
                        let items = v.as_array().cloned().unwrap_or_else(|| vec![v]);
                        for item in items {
                            let mut r = row.clone();
                            r.insert(alias.clone(), item);
                            next.push(r);
                        }
                    }
                    self.rows = next;
                }
                Step::Project { items } => {
                    let mut next = Vec::new();
                    for row in &self.rows {
                        let mut r = Row::new();
                        for item in items {
                            r.insert(item_key(item), resolve_item(item, row));
                        }
                        next.push(r);
                    }
                    self.rows = next;
                }
                Step::Call { operator, document, variables } => {
                    if *operator == OpKind::Subscription {
                        return Ok(());
                    }
                    let data = transport.execute(*operator, document, variables.clone()).await?;
                    self.ingest_call_result(&data);
                }
            }
            Ok(())
        }

        /// 🪝 `CALL` binds the whole (unwrapped) response payload to a single conventional `result`
        /// row/var — Jack's `CALL` grammar has no `YIELD` clause, so per-field extraction is done by
        /// `RETURN result.<field>` (one property-access level) instead of `YIELD <field> AS ...`.
        fn ingest_call_result(&mut self, data: &Value) {
            let mut row = Row::new();
            row.insert("result".into(), data.clone());
            self.rows = vec![row];
        }

        fn finish(&self, items: Option<&[jack::ReturnItem]>) -> QueryResult {
            let Some(items) = items else {
                return QueryResult { columns: vec![], rows: self.rows.iter().map(|r| Value::Object(r.iter().map(|(k, v)| (k.clone(), v.clone())).collect())).collect() };
            };
            let mut columns = Vec::new();
            let mut rows = Vec::new();
            for row in &self.rows {
                let mut out = Row::new();
                for item in items {
                    let key = item_key(item);
                    if !columns.contains(&key) {
                        columns.push(key.clone());
                    }
                    out.insert(key, resolve_item(item, row));
                }
                rows.push(Value::Object(out.into_iter().collect()));
            }
            QueryResult { columns, rows }
        }
    }

    /// 🔑️ `RETURN`/`WITH` column key for a Jack `ReturnItem` — Jack has no `AS` aliasing on return
    /// items, so the key is always derived from the item itself (`var` or `var.prop`).
    fn item_key(item: &jack::ReturnItem) -> String {
        match item {
            jack::ReturnItem::Var(v) => v.clone(),
            jack::ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        }
    }

    fn resolve_item(item: &jack::ReturnItem, row: &Row) -> Value {
        match item {
            jack::ReturnItem::Var(v) => row.get(v).cloned().unwrap_or(Value::Null),
            jack::ReturnItem::Property { var, prop } => row.get(var).and_then(|v| v.get(prop)).cloned().unwrap_or(Value::Null),
        }
    }

    fn extract_rows(data: &Value, bind: &super::planner::BindSpec) -> Result<Vec<Row>, ArchitectError> {
        let root = data.get("data").unwrap_or(data);
        let anchor_path = bind.paths.get(&bind.anchor_var).ok_or_else(|| ArchitectError::Execute("bind missing anchor path".into()))?;
        let anchors = read_path(root, anchor_path);
        if anchors.is_empty() {
            return Ok(vec![]);
        }
        let mut rows = Vec::new();
        for anchor in anchors {
            let mut row = Row::new();
            row.insert(bind.anchor_var.clone(), anchor.clone());
            for var in bind.paths.keys() {
                if var == &bind.anchor_var {
                    continue;
                }
                if let Some(v) = row.get(&bind.anchor_var) {
                    row.insert(var.clone(), v.clone());
                }
            }
            rows.push(row);
        }
        Ok(rows)
    }

    fn read_path(root: &Value, path: &JsonPath) -> Vec<Value> {
        let mut cur = vec![root.clone()];
        for seg in &path.segments {
            let mut next = Vec::new();
            for v in cur {
                match seg {
                    PathSeg::Field { name } => {
                        if let Some(c) = v.get(name) {
                            next.push(c.clone());
                        }
                    }
                    PathSeg::ConnectionEdges => {
                        if let Some(edges) = v.get("edges").and_then(|e| e.as_array()) {
                            for e in edges {
                                next.push(e.clone());
                            }
                        }
                    }
                    PathSeg::ConnectionNode => {
                        if let Some(node) = v.get("node") {
                            next.push(node.clone());
                        }
                    }
                }
            }
            cur = next;
            if cur.is_empty() {
                break;
            }
        }
        cur
    }

    fn join_rows(rows: &[Row], on_var: &str, key: &str) -> Vec<Row> {
        let mut by_key: BTreeMap<String, Vec<Row>> = BTreeMap::new();
        for row in rows {
            let Some(ent) = row.get(on_var) else { continue };
            let id = ent.get(key).and_then(|v| v.as_str()).unwrap_or("").to_string();
            by_key.entry(id).or_default().push(row.clone());
        }
        let mut out = Vec::new();
        for group in by_key.values() {
            if group.len() == 1 {
                out.push(group[0].clone());
                continue;
            }
            let mut merged = group[0].clone();
            for other in &group[1..] {
                for (k, v) in other {
                    merged.entry(k.clone()).or_insert_with(|| v.clone());
                }
            }
            out.push(merged);
        }
        out
    }

    fn cartesian_merge(left: &[Row], right: &[Row]) -> Vec<Row> {
        if left.is_empty() {
            return right.to_vec();
        }
        if right.is_empty() {
            return left.to_vec();
        }
        let mut out = Vec::new();
        for a in left {
            for b in right {
                let mut m = a.clone();
                for (k, v) in b {
                    m.insert(k.clone(), v.clone());
                }
                out.push(m);
            }
        }
        out
    }

    fn eval_bool(expr: &jack::Expr, row: &Row) -> bool {
        match expr {
            jack::Expr::Eq { var, prop, value } => field_eq(row, var, prop, value),
            jack::Expr::Ne { var, prop, value } => !field_eq(row, var, prop, value),
            jack::Expr::And(a, b) => eval_bool(a, row) && eval_bool(b, row),
            jack::Expr::Or(a, b) => eval_bool(a, row) || eval_bool(b, row),
        }
    }

    fn field_eq(row: &Row, var: &str, prop: &str, value: &PropertyValue) -> bool {
        let Some(actual) = row.get(var).and_then(|v| v.get(prop)) else { return false };
        json_eq(actual, &super::property_value_to_json(value))
    }

    fn json_eq(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
            _ => a == b,
        }
    }
}
//#endregion 🔖️Executor

//#region 🔖️Api
mod api {
    use super::errors::ArchitectError;
    use super::executor::{Executor, QueryResult};
    use super::planner::{plan_query, OpPlan};
    use super::transport::Transport;
    use futures_util::StreamExt;
    use mathematical_graph_dsl as jack;

    /// 🔍️ Parse architect source — literally Jack's own parser (`mathematical_graph_dsl::parse`);
    /// architect no longer carries any lexer/parser/AST of its own.
    pub fn parse(text: &str) -> Result<jack::Query, ArchitectError> {
        jack::parse(text).map_err(|e| ArchitectError::Parse(e.to_string()))
    }

    /// @emoji 🧭️ Plan Jack's AST.
    pub fn plan(ast: &jack::Query) -> Result<OpPlan, ArchitectError> {
        plan_query(ast)
    }

    /// @emoji 📜️ Compile to `OpPlan` JSON-friendly plan.
    pub fn compile(text: &str) -> Result<OpPlan, ArchitectError> {
        plan(&parse(text)?)
    }

    /// @emoji ▶️ Parse, plan, and execute end-to-end.
    pub async fn run(text: &str, transport: &dyn Transport) -> Result<QueryResult, ArchitectError> {
        let ast = parse(text)?;
        let plan = plan(&ast)?;
        if plan.steps.iter().any(|s| matches!(s, super::planner::Step::Call { operator: super::transport::OpKind::Subscription, .. })) {
            let mut stream = Executor::run_subscription(&plan, transport).await?;
            if let Some(first) = stream.next().await {
                return first;
            }
            return Err(ArchitectError::Execute("empty subscription stream".into()));
        }
        Executor::run(&plan, transport).await
    }
}
//#endregion 🔖️Api

#[cfg(target_arch = "wasm32")]
mod wasm_api {
    use super::api;
    use super::planner::{OpPlan, PathSeg, Step};
    use super::transport::JsTransport;
    use mathematical_graph_dsl as jack;
    use mathematical_graph_manifest::PropertyValue;
    use serde_json::{json, Value};
    use wasm_bindgen::prelude::*;

    fn export_property_value(v: &PropertyValue) -> Value {
        super::property_value_to_json(v)
    }

    fn export_expr(expr: &jack::Expr) -> Value {
        match expr {
            jack::Expr::Eq { var, prop, value } => json!({ "kind": "eq", "var": var, "prop": prop, "value": export_property_value(value) }),
            jack::Expr::Ne { var, prop, value } => json!({ "kind": "ne", "var": var, "prop": prop, "value": export_property_value(value) }),
            jack::Expr::And(left, right) => json!({ "kind": "and", "left": export_expr(left), "right": export_expr(right) }),
            jack::Expr::Or(left, right) => json!({ "kind": "or", "left": export_expr(left), "right": export_expr(right) }),
        }
    }

    fn export_return_item(item: &jack::ReturnItem) -> Value {
        match item {
            jack::ReturnItem::Var(name) => json!({ "kind": "var", "name": name }),
            jack::ReturnItem::Property { var, prop } => json!({ "kind": "property", "var": var, "prop": prop }),
        }
    }

    fn export_path_seg(seg: &PathSeg) -> Value {
        match seg {
            PathSeg::Field { name } => json!({ "kind": "field", "name": name }),
            PathSeg::ConnectionEdges => json!({ "kind": "connectionEdges" }),
            PathSeg::ConnectionNode => json!({ "kind": "connectionNode" }),
        }
    }

    /// @emoji 📤️ Wasm-safe `OpPlan` JSON, built by hand rather than via `serde` derive since Jack's
    /// `Expr`/`ReturnItem` (reused directly, no architect-owned mirror) don't implement `Serialize`.
    fn export_plan(plan: &OpPlan) -> Value {
        let steps: Vec<Value> = plan
            .steps
            .iter()
            .map(|step| match step {
                Step::GraphQl { operator, document, variables, bind } => {
                    let mut paths = serde_json::Map::new();
                    for (k, p) in &bind.paths {
                        paths.insert(k.clone(), Value::Array(p.segments.iter().map(export_path_seg).collect()));
                    }
                    json!({
                        "kind": "graphQl",
                        "operator": format!("{operator:?}"),
                        "document": document,
                        "variables": variables,
                        "bind": {
                            "anchorVar": bind.anchor_var,
                            "anchorLabel": bind.anchor_label,
                            "paths": paths,
                        },
                    })
                }
                Step::Join { on_var, key } => json!({ "kind": "join", "onVar": on_var, "key": key }),
                Step::Filter { expr } => json!({ "kind": "filter", "expr": export_expr(expr) }),
                Step::Unwind { source_var, source_prop, alias } => json!({
                    "kind": "unwind",
                    "sourceVar": source_var,
                    "sourceProp": source_prop,
                    "alias": alias,
                }),
                Step::Project { items } => json!({
                    "kind": "project",
                    "items": items.iter().map(export_return_item).collect::<Vec<_>>(),
                }),
                Step::Call { operator, document, variables } => json!({
                    "kind": "call",
                    "operator": format!("{operator:?}"),
                    "document": document,
                    "variables": variables,
                }),
            })
            .collect();
        json!({
            "steps": steps,
            "returnItems": plan.return_items.as_ref().map(|items| items.iter().map(export_return_item).collect::<Vec<_>>()),
        })
    }

    /// @emoji 🌐️ Compile architect query to JSON plan (wasm).
    #[wasm_bindgen(js_name = architectCompile)]
    pub fn architect_compile(query: &str) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();
        match api::compile(query) {
            Ok(p) => serde_wasm_bindgen::to_value(&export_plan(&p)).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// @emoji 🌐️ Run architect query via JS transport callbacks (wasm).
    #[wasm_bindgen(js_name = architectRun)]
    pub async fn architect_run(query: &str, execute_fn: js_sys::Function, subscribe_fn: js_sys::Function) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();
        let transport = JsTransport::new(execute_fn, subscribe_fn);
        match api::run(query, &transport).await {
            Ok(r) => serde_wasm_bindgen::to_value(&r).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mathematical_graph_dsl as jack;
    use std::collections::HashMap;
    use std::path::PathBuf;

    //#region 🧪️architect_cases
    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../fixture")
    }

    fn architect_cases_doc() -> serde_json::Value {
        let path = fixtures_dir().join("architect.cases.compose.json");
        serde_json::from_str(&std::fs::read_to_string(&path).expect("read architect.cases.compose.json")).expect("parse cases")
    }

    fn architect_harness_kit() -> serde_json::Value {
        let path = fixtures_dir().join("architect.harness.kit.compose.json");
        serde_json::from_str(&std::fs::read_to_string(&path).expect("read architect.harness.kit.compose.json")).expect("parse harness kit")
    }

    fn case_rows(doc: &serde_json::Value) -> &[serde_json::Value] {
        doc["cases"].as_array().expect("cases array").as_slice()
    }

    fn cases_for_tier<'a>(doc: &'a serde_json::Value, tier: &str) -> Vec<&'a serde_json::Value> {
        case_rows(doc).iter().filter(|c| c["tier"].as_str() == Some(tier)).collect()
    }

    fn column_values(result: &QueryResult, column: &str) -> Vec<serde_json::Value> {
        result.rows.iter().filter_map(|row| row.get(column).cloned()).collect()
    }

    fn assert_query_expect(case_name: &str, result: &QueryResult, expect: &serde_json::Value) {
        if let Some(cols) = expect.get("columns").and_then(|v| v.as_array()) {
            let exp: Vec<String> = cols.iter().filter_map(|v| v.as_str().map(str::to_string)).collect();
            assert_eq!(result.columns, exp, "case {case_name} columns");
        }
        if let Some(min) = expect.get("minRows").and_then(|v| v.as_u64()) {
            assert!(result.rows.len() >= min as usize, "case {case_name} minRows");
        }
        if let Some(max) = expect.get("maxRows").and_then(|v| v.as_u64()) {
            assert!(result.rows.len() <= max as usize, "case {case_name} maxRows");
        }
        if let Some(rows) = expect.get("rowContains").and_then(|v| v.as_array()) {
            for want in rows {
                let obj = want.as_object().expect("rowContains object");
                let hit = result.rows.iter().any(|row| obj.iter().all(|(k, v)| row.get(k).is_some_and(|a| a == v)));
                assert!(hit, "case {case_name} rowContains {want}");
            }
        }
        if let Some(include) = expect.get("valuesInclude").and_then(|v| v.as_object()) {
            for (col, vals) in include {
                let got = column_values(result, col);
                for v in vals.as_array().expect("valuesInclude array") {
                    assert!(got.iter().any(|g| g == v), "case {case_name} valuesInclude {col} missing {v}");
                }
            }
        }
        if let Some(exclude) = expect.get("valuesExclude").and_then(|v| v.as_object()) {
            for (col, vals) in exclude {
                let got = column_values(result, col);
                for v in vals.as_array().expect("valuesExclude array") {
                    assert!(!got.iter().any(|g| g == v), "case {case_name} valuesExclude {col} still has {v}");
                }
            }
        }
    }

    fn register_canned_steps(plan: &OpPlan, canned: &[serde_json::Value], responses: &mut HashMap<String, serde_json::Value>) {
        let mut idx = 0usize;
        for step in &plan.steps {
            match step {
                planner::Step::GraphQl { document, operator, .. } | planner::Step::Call { document, operator, .. } => {
                    let payload = canned.get(idx).expect("canned step payload").clone();
                    responses.insert(format!("{operator:?}:{document}"), payload.clone());
                    responses.insert(document.clone(), payload);
                    idx += 1;
                }
                _ => {}
            }
        }
        assert_eq!(idx, canned.len(), "canned response count mismatch");
    }

    fn memory_transport_for_case(case: &serde_json::Value) -> MemoryTransport {
        let query = case["query"].as_str().expect("query");
        let plan = compile(query).expect("compile");
        let canned = case["graphqlResponses"].as_array().expect("graphqlResponses");
        let mut responses = HashMap::new();
        register_canned_steps(&plan, canned, &mut responses);
        MemoryTransport::new(responses)
    }

    fn assert_plan_expect(name: &str, plan: &OpPlan, expect: &serde_json::Value) {
        if let Some(min) = expect.get("minGraphQlSteps").and_then(|v| v.as_u64()) {
            let n = plan.steps.iter().filter(|s| matches!(s, planner::Step::GraphQl { .. })).count();
            assert!(n >= min as usize, "case {name} graphql steps");
        }
        if let Some(vars) = expect.get("joinVars").and_then(|v| v.as_array()) {
            for v in vars {
                let var = v.as_str().unwrap();
                assert!(
                    plan.steps.iter().any(|s| matches!(
                        s,
                        planner::Step::Join { on_var, .. } if on_var == var
                    )),
                    "case {name} join on {var}"
                );
            }
        }
        if let Some(min) = expect.get("minFilterSteps").and_then(|v| v.as_u64()) {
            let n = plan.steps.iter().filter(|s| matches!(s, planner::Step::Filter { .. })).count();
            assert!(n >= min as usize, "case {name} filter steps");
        }
    }

    #[test]
    fn architect_cases_fixture_contract() {
        let doc = architect_cases_doc();
        assert_eq!(doc["kit"].as_str(), Some("architect.harness.kit.compose.json"));
        let cases = case_rows(&doc);
        assert_eq!(cases.len(), 13);
        for tier in ["e2e", "memory", "plan", "parse"] {
            assert!(!cases_for_tier(&doc, tier).is_empty(), "missing tier {tier}");
        }
        for case in cases {
            assert!(case.get("name").and_then(|v| v.as_str()).is_some());
            assert!(case.get("query").and_then(|v| v.as_str()).is_some());
            assert!(case.get("expect").is_some());
            match case["tier"].as_str().expect("tier") {
                "memory" => assert!(case.get("graphqlResponses").is_some(), "memory needs graphqlResponses"),
                "e2e" if case.get("runtime").and_then(|v| v.as_str()) != Some("empty") => {}
                _ => {}
            }
        }
        let kit = architect_harness_kit();
        assert_eq!(kit["name"].as_str(), Some("Architect Harness"));
        let topos = kit["typologies"]["items"].as_array().expect("typologies items");
        let total_designs: usize = topos.iter().map(|t| t["designs"]["items"].as_array().map_or(0, Vec::len)).sum();
        let total_types: usize = topos.iter().map(|t| t["types"]["items"].as_array().map_or(0, Vec::len)).sum();
        assert_eq!(total_designs, 2, "total designs across typologies");
        assert_eq!(total_types, 3, "total types across typologies");
    }

    #[test]
    fn architect_cases_plan_and_parse_tiers() {
        let doc = architect_cases_doc();
        for case in cases_for_tier(&doc, "parse") {
            let name = case["name"].as_str().unwrap();
            let q = parse(case["query"].as_str().unwrap()).expect("parse");
            if case["expect"].get("hasReturn").and_then(|v| v.as_bool()) == Some(true) {
                assert!(q.clauses.iter().any(|c| matches!(c, jack::Clause::Return(_))), "case {name}");
            }
        }
        for case in cases_for_tier(&doc, "plan") {
            let name = case["name"].as_str().unwrap();
            let p = plan(&parse(case["query"].as_str().unwrap()).unwrap()).unwrap();
            assert_plan_expect(name, &p, &case["expect"]);
        }
    }

    #[tokio::test]
    async fn architect_cases_memory_suite() {
        let doc = architect_cases_doc();
        for case in cases_for_tier(&doc, "memory") {
            let name = case["name"].as_str().unwrap();
            let query = case["query"].as_str().unwrap();
            let transport = memory_transport_for_case(case);
            let result = run(query, &transport).await.unwrap_or_else(|e| panic!("case {name}: {e}"));
            assert_query_expect(name, &result, &case["expect"]);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn compose_schema_for_case(case: &serde_json::Value) -> compose::gql::AppSchema {
        if case.get("runtime").and_then(|v| v.as_str()) == Some("empty") {
            return compose::gql::build_schema_for(compose::worker::ParentStore::spawn().await);
        }
        let kit = architect_harness_kit();
        let rt = compose::worker::ParentStore::spawn_wip_overlay_from_initial_kit_projection_json(kit).await.expect("hydrate architect harness");
        compose::gql::build_schema_for(rt)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[tokio::test]
    async fn architect_cases_e2e_suite() {
        let doc = architect_cases_doc();
        for case in cases_for_tier(&doc, "e2e") {
            let name = case["name"].as_str().unwrap();
            let query = case["query"].as_str().unwrap();
            let transport = ComposeTransport::new(compose_schema_for_case(case).await);
            let result = run(query, &transport).await.unwrap_or_else(|e| panic!("case {name}: {e}"));
            assert_query_expect(name, &result, &case["expect"]);
        }
    }
    //#endregion 🧪️architect_cases

    //#region 🧪️unified_jack_syntax_round_trips
    // 🌱️ Architect's `parse()` is literally `mathematical_graph_dsl::parse` now — these tests
    // verify that identity directly, and that Jack's own formatter/parser pair (shared with every
    // other Jack consumer in the repo) round-trips the query shapes architect relies on.
    #[test]
    fn unified_jack_syntax_parse_is_identical_to_jack_parse() {
        let src = "MATCH (d:Design) WHERE d.name = \"Nakagin Capsule Tower\" RETURN d.name";
        assert_eq!(parse(src).expect("architect parse"), jack::parse(src).expect("jack parse"));
    }

    #[test]
    fn unified_jack_syntax_round_trips_through_shared_formatter() {
        let src = "MATCH (d:Design)--[:HAS]->(p:Piece) WHERE d.name = \"Nakagin Capsule Tower\" RETURN d.name, p.name";
        let formatted = jack::format(src).expect("format");
        let reparsed = jack::parse(&formatted).expect("reparse formatted");
        assert_eq!(parse(src).expect("parse original"), reparsed, "formatting must not change the parsed AST");
    }

    #[test]
    fn unified_jack_syntax_call_and_unwind_round_trip() {
        let src = "CALL installProjection(\"s1\", \"{}\") RETURN result";
        let formatted = jack::format(src).expect("format");
        assert_eq!(parse(src).expect("parse"), jack::parse(&formatted).expect("reparse"));

        let unwind_src = "MATCH (d:Design) UNWIND d.name AS n RETURN n";
        let formatted_unwind = jack::format(unwind_src).expect("format");
        assert_eq!(parse(unwind_src).expect("parse"), jack::parse(&formatted_unwind).expect("reparse"));
    }
    //#endregion 🧪️unified_jack_syntax_round_trips

    //#region 🧪️unit_coverage
    #[test]
    fn parser_escaped_string_literal_unescapes_embedded_quote() {
        let q = parse(r#"MATCH (d:Design) WHERE d.name = "a\"b" RETURN d"#).expect("parse");
        let jack::Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match clause") };
        assert_eq!(patterns[0].nodes[0].kind, "Design");
        let jack::Clause::Where(jack::Expr::Eq { value, .. }) = &q.clauses[1] else { panic!("expected where eq") };
        assert_eq!(value.as_str(), Some("a\"b"));
    }

    #[test]
    fn parser_negative_number_literal_in_where() {
        let q = parse("MATCH (t:Type) WHERE t.x = -1.5 RETURN t.x").expect("parse");
        let jack::Clause::Where(jack::Expr::Eq { value, .. }) = &q.clauses[1] else { panic!("expected where eq") };
        assert_eq!(value.as_f64(), Some(-1.5));
    }

    #[test]
    fn parser_and_binds_predicates_together() {
        let q = parse("MATCH (t:Type) WHERE t.a = 1 AND t.b = 2 RETURN t").expect("parse");
        let jack::Clause::Where(expr) = &q.clauses[1] else { panic!("expected where") };
        assert!(matches!(expr, jack::Expr::And(_, _)));
    }

    #[test]
    fn parser_undirected_relationship() {
        let q = parse("MATCH (a:Type)--[:HAS]--(b:Connector) RETURN a").expect("parse");
        let jack::Clause::Match(patterns) = &q.clauses[0] else { panic!("expected match clause") };
        let edge = patterns[0].edge.as_ref().expect("expected edge");
        assert!(!edge.directed, "-- both sides must parse as undirected");
        assert_eq!(edge.kind.as_deref(), Some("HAS"));
    }

    #[test]
    fn parser_rejects_trailing_input_after_query() {
        let err = parse("MATCH (n:Type) RETURN n EXTRA_JUNK").unwrap_err();
        let ArchitectError::Parse(msg) = err else { panic!("expected parse error") };
        assert!(msg.contains("EXTRA_JUNK") || msg.to_ascii_lowercase().contains("expected"), "message: {msg}");
    }

    #[test]
    fn parser_call_clause_with_positional_args() {
        let q = parse("CALL installProjection(\"s1\", \"{}\")").expect("parse");
        let jack::Clause::Call(c) = &q.clauses[0] else { panic!("expected call clause") };
        assert_eq!(c.name, "installProjection");
        assert_eq!(c.args.len(), 2);
    }

    #[test]
    fn planner_errors_on_unknown_label() {
        let q = parse("MATCH (n:Bogus) RETURN n").expect("parse");
        let err = plan(&q).unwrap_err();
        let ArchitectError::Plan(msg) = err else { panic!("expected plan error") };
        assert!(msg.contains("unknown label"), "message: {msg}");
    }

    #[test]
    fn planner_errors_when_anchor_label_not_session_root() {
        let q = parse("MATCH (p:Piece) RETURN p").expect("parse");
        let err = plan(&q).unwrap_err();
        let ArchitectError::Plan(msg) = err else { panic!("expected plan error") };
        assert!(msg.contains("must be Kit, Design, or Type"), "message: {msg}");
    }

    #[test]
    fn planner_call_step_resolves_store_alias_and_json_variables() {
        let q = parse(r#"CALL installProjection("s1", "{}")"#).expect("parse");
        let p = plan(&q).expect("plan");
        let planner::Step::Call { variables, .. } = &p.steps[0] else { panic!("expected call step") };
        assert_eq!(variables.get("storeId").and_then(|v| v.as_str()), Some("s1"));
        assert_eq!(variables.get("json").and_then(|v| v.as_str()), Some("{}"));
    }

    #[test]
    fn schema_predicate_parse_is_case_insensitive_and_rejects_unknown() {
        assert_eq!(schema::Predicate::parse("has"), Some(schema::Predicate::Has));
        assert_eq!(schema::Predicate::parse("Owns"), Some(schema::Predicate::Owns));
        assert_eq!(schema::Predicate::parse("parent"), Some(schema::Predicate::Parent));
        assert_eq!(schema::Predicate::parse("bogus"), None);
    }

    #[test]
    fn schema_resolve_edge_errors_for_unknown_predicate_combo() {
        let err = schema::resolve_edge(schema::Label::Kit, schema::Predicate::Owns, schema::Label::Design).unwrap_err();
        let ArchitectError::Plan(msg) = err else { panic!("expected plan error") };
        assert!(msg.contains("no edge"), "message: {msg}");
    }

    #[test]
    fn schema_resolve_edge_parent_and_child_are_distinct_predicates() {
        let parent_edge = schema::resolve_edge(schema::Label::Connection, schema::Predicate::Parent, schema::Label::Side).expect("parent edge");
        assert_eq!(parent_edge.field, "parent");
        let child_edge = schema::resolve_edge(schema::Label::Connection, schema::Predicate::Child, schema::Label::Side).expect("child edge");
        assert_eq!(child_edge.field, "child");
    }

    #[test]
    fn schema_node_label_and_rel_predicate_success_and_error_paths() {
        let labeled = jack::PatternNode { var: "d".into(), kind: "Design".into(), port: None };
        assert_eq!(schema::node_label(&labeled).unwrap(), schema::Label::Design);

        let unknown = jack::PatternNode { var: "d".into(), kind: "Bogus".into(), port: None };
        let err = schema::node_label(&unknown).unwrap_err();
        let ArchitectError::Plan(msg) = err else { panic!("expected plan error") };
        assert!(msg.contains("unknown label"));

        assert_eq!(schema::rel_predicate(Some("HAS")).unwrap(), schema::Predicate::Has);
        let err = schema::rel_predicate(None).unwrap_err();
        let ArchitectError::Plan(msg) = err else { panic!("expected plan error") };
        assert!(msg.contains("requires a predicate"));
    }

    #[test]
    fn schema_entity_scalar_fields_default_branch_for_unlisted_label() {
        assert_eq!(schema::entity_scalar_fields(schema::Label::Blueprint), &["id", "hash"]);
        assert_eq!(schema::entity_scalar_fields(schema::Label::Group), &["id", "hash"]);
    }

    #[test]
    fn schema_resolve_call_errors_for_unknown_target() {
        let err = schema::resolve_call("nope").unwrap_err();
        let ArchitectError::Plan(msg) = err else { panic!("expected plan error") };
        assert!(msg.contains("unknown CALL target"));
    }

    #[test]
    fn schema_call_variables_empty_for_non_store_action() {
        let vars = schema::call_variables("sessionEnd", &[]);
        assert_eq!(vars, serde_json::json!({}));
    }

    #[tokio::test]
    async fn executor_cartesian_merge_combines_independent_patterns() {
        let query = "MATCH (d:Design), (t:Type) RETURN d.name, t.name";
        let op_plan = compile(query).expect("compile");
        let design_payload = serde_json::json!({
            "data": { "session": { "stores": { "edges": [ { "node": { "wip": { "theKit": { "kit": {
                "hasDesigns": { "edges": [
                    { "node": { "id": "d1", "hash": "h1", "name": "DesignA" } },
                    { "node": { "id": "d2", "hash": "h2", "name": "DesignB" } }
                ] }
            } } } } } ] } } }
        });
        let type_payload = serde_json::json!({
            "data": { "session": { "stores": { "edges": [ { "node": { "wip": { "theKit": { "kit": {
                "hasTypes": { "edges": [
                    { "node": { "id": "t1", "hash": "ht1", "name": "TypeA" } },
                    { "node": { "id": "t2", "hash": "ht2", "name": "TypeB" } },
                    { "node": { "id": "t3", "hash": "ht3", "name": "TypeC" } }
                ] }
            } } } } } ] } } }
        });
        let mut responses = HashMap::new();
        register_canned_steps(&op_plan, &[design_payload, type_payload], &mut responses);
        let transport = MemoryTransport::new(responses);
        let result = run(query, &transport).await.expect("run");
        assert_eq!(result.rows.len(), 6);
        assert_eq!(result.columns, vec!["d.name".to_string(), "t.name".to_string()]);
    }

    #[tokio::test]
    async fn executor_unwind_flattens_array_value() {
        let query = "CALL sessionEnd() UNWIND result.items AS x RETURN x";
        let op_plan = compile(query).expect("compile");
        let mut responses = HashMap::new();
        register_canned_steps(&op_plan, &[serde_json::json!({"items": [1, 2, 3]})], &mut responses);
        let transport = MemoryTransport::new(responses);
        let result = run(query, &transport).await.expect("run");
        let xs: Vec<_> = result.rows.iter().filter_map(|r| r.get("x").and_then(|v| v.as_i64())).collect();
        assert_eq!(xs, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn executor_unwind_wraps_non_array_value_as_single_item() {
        let query = "CALL sessionEnd() UNWIND result AS x RETURN x";
        let op_plan = compile(query).expect("compile");
        let mut responses = HashMap::new();
        register_canned_steps(&op_plan, &[serde_json::json!("solo")], &mut responses);
        let transport = MemoryTransport::new(responses);
        let result = run(query, &transport).await.expect("run");
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].get("x").and_then(|v| v.as_str()), Some("solo"));
    }

    #[tokio::test]
    async fn executor_call_result_binds_whole_payload_under_result_var() {
        let query = "CALL sessionEnd() RETURN result";
        let op_plan = compile(query).expect("compile");
        let payload = serde_json::json!({"data": {"session": {"end": {"ok": true}}}});
        let mut responses = HashMap::new();
        register_canned_steps(&op_plan, std::slice::from_ref(&payload), &mut responses);
        let transport = MemoryTransport::new(responses);
        let result = run(query, &transport).await.expect("run");
        assert_eq!(result.rows[0].get("result"), Some(&payload));
    }

    #[tokio::test]
    async fn executor_call_result_property_access_reaches_one_level_deep() {
        let query = "CALL sessionEnd() RETURN result.data";
        let op_plan = compile(query).expect("compile");
        let mut responses = HashMap::new();
        register_canned_steps(&op_plan, &[serde_json::json!({"data": {"ok": true}})], &mut responses);
        let transport = MemoryTransport::new(responses);
        let result = run(query, &transport).await.expect("run");
        assert_eq!(result.rows[0].get("result.data"), Some(&serde_json::json!({"ok": true})));
    }

    #[tokio::test]
    async fn executor_memory_transport_missing_canned_response_errors() {
        let transport = MemoryTransport::new(HashMap::new());
        let err = run("MATCH (t:Type) RETURN t.name", &transport).await.unwrap_err();
        let ArchitectError::Transport(TransportError::Msg(msg)) = err else { panic!("expected transport error") };
        assert!(msg.contains("no canned response"), "message: {msg}");
    }

    #[tokio::test]
    async fn executor_run_subscription_errors_when_plan_has_no_subscription_call() {
        let op_plan = compile("MATCH (t:Type) RETURN t.name").expect("compile");
        let transport = MemoryTransport::new(HashMap::new());
        let err = match Executor::run_subscription(&op_plan, &transport).await {
            Ok(_) => panic!("expected execute error"),
            Err(e) => e,
        };
        let ArchitectError::Execute(msg) = err else { panic!("expected execute error") };
        assert!(msg.contains("plan has no subscription CALL"));
    }

    #[tokio::test]
    async fn api_run_dispatches_subscription_operation_call() {
        let query = "CALL subscriptionOperation() RETURN result";
        let op_plan = compile(query).expect("compile");
        let mut responses = HashMap::new();
        register_canned_steps(&op_plan, &[serde_json::json!({"data": {"operation": {"id": "op1", "hash": "h1"}}})], &mut responses);
        let transport = MemoryTransport::new(responses);
        let result = run(query, &transport).await.expect("run");
        assert_eq!(result.rows[0].get("result").and_then(|v| v.get("data")).and_then(|v| v.get("operation")).and_then(|v| v.get("id")).and_then(|v| v.as_str()), Some("op1"));
    }

    #[test]
    fn errors_display_messages_match_thiserror_format() {
        assert_eq!(ArchitectError::Parse("boom".into()).to_string(), "parse: boom");
        assert_eq!(ArchitectError::Plan("boom".into()).to_string(), "plan: boom");
        assert_eq!(ArchitectError::Execute("boom".into()).to_string(), "execute: boom");
        let transport_err: ArchitectError = TransportError::Msg("boom".into()).into();
        assert_eq!(transport_err.to_string(), "transport: boom");
    }
    //#endregion 🧪️unit_coverage
}
