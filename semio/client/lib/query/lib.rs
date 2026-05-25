//! @emoji 🏛️ `architect` — Cypher-inspired semio query language: parse, plan GraphQL, execute via `Transport`.

#![allow(clippy::too_many_lines)]

pub use api::{compile, parse, plan, run};
pub use executor::QueryResult;
pub use errors::ArchitectError;
pub use executor::Executor;
pub use planner::OpPlan;
pub use transport::{MemoryTransport, OpKind, Transport, TransportError};

#[cfg(target_arch = "wasm32")]
pub use wasm_api::{architect_compile, architect_run};

//#region 🔖Errors
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
//#endregion 🔖Errors

//#region 🔖Ast
mod ast {
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    /// @emoji 🌳 Parsed architect statement.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Query {
        pub clauses: Vec<Clause>,
        pub return_clause: Option<ReturnClause>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum Clause {
        Match(MatchClause),
        With(WithClause),
        Unwind(UnwindClause),
        Call(CallClause),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MatchClause {
        pub patterns: Vec<Pattern>,
        pub where_expr: Option<Expr>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct WithClause {
        pub projections: Vec<ProjectionItem>,
        pub where_expr: Option<Expr>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct UnwindClause {
        pub source: Expr,
        pub alias: String,
        pub where_expr: Option<Expr>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CallClause {
        pub action_id: String,
        pub args: BTreeMap<String, serde_json::Value>,
        pub yield_items: Vec<YieldItem>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ReturnClause {
        pub projections: Vec<ProjectionItem>,
        pub order_by: Option<Expr>,
        pub limit: Option<usize>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ProjectionItem {
        pub expr: Expr,
        pub alias: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct YieldItem {
        pub key: String,
        pub alias: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Pattern {
        pub elements: Vec<PatternElement>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum PatternElement {
        Node(NodePattern),
        Rel(RelPattern),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NodePattern {
        pub var_name: Option<String>,
        pub label: Option<String>,
        pub props: BTreeMap<String, serde_json::Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RelPattern {
        pub types: Vec<String>,
        pub direction: RelDirection,
        pub props: BTreeMap<String, serde_json::Value>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum RelDirection {
        Out,
        In,
        Undirected,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum Expr {
        Const(serde_json::Value),
        Var { name: String },
        Field { object: Box<Expr>, name: String },
        BinOp { op: BinOp, left: Box<Expr>, right: Box<Expr> },
        UnaryNeg(Box<Expr>),
        And(Vec<Expr>),
        Or(Vec<Expr>),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum BinOp {
        Eq,
        Ne,
        Lt,
        Le,
        Gt,
        Ge,
        Add,
        Sub,
        Mul,
        Div,
    }
}
//#endregion 🔖Ast

//#region 🔖Parser
mod parser {
    use super::ast::*;
    use super::errors::ArchitectError;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, tag_no_case};
    use nom::character::complete::{alphanumeric1, char, digit1, multispace0};
    use nom::combinator::{cut, map, opt, recognize, value};
    use nom::multi::{many0, separated_list0, separated_list1};
    use nom::sequence::{delimited, pair, preceded};
    use nom::{IResult, Parser};
    use std::collections::BTreeMap;

    fn ws<'a, O, P>(p: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
    where
        P: Parser<&'a str, O, nom::error::Error<&'a str>>,
    {
        preceded(multispace0, p)
    }

    fn kw<'a>(s: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
        ws(tag_no_case(s))
    }

    fn ident(input: &str) -> IResult<&str, &str> {
        ws(recognize(pair(
            alt((alphanumeric1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )))(input)
    }

    fn string_lit(input: &str) -> IResult<&str, String> {
        let (rest, inner) = ws(alt((
            delimited(char('"'), recognize(many0(alt((tag("\\\""), tag("\\\\"), nom::bytes::complete::is_not("\"\\"))))), char('"')),
            delimited(char('\''), recognize(many0(alt((tag("\\'"), tag("\\\\"), nom::bytes::complete::is_not("'\\"))))), char('\'')),
        )))(input)?;
        let unescaped = if inner.contains('\\') {
            inner.replace("\\'", "'").replace("\\\"", "\"")
        } else {
            inner.to_string()
        };
        Ok((rest, unescaped))
    }

    fn number_lit(input: &str) -> IResult<&str, serde_json::Value> {
        map(
            ws(recognize(pair(
                opt(char('-')),
                alt((recognize(pair(digit1, pair(tag("."), digit1))), digit1)),
            ))),
            |s: &str| {
            if s.contains('.') {
                serde_json::Value::from(s.parse::<f64>().unwrap_or(0.0))
            } else {
                serde_json::Value::from(s.parse::<i64>().unwrap_or(0))
            }
        })(input)
    }

    fn literal_value(input: &str) -> IResult<&str, serde_json::Value> {
        alt((map(string_lit, serde_json::Value::String), number_lit))(input)
    }

    fn object_literal(input: &str) -> IResult<&str, BTreeMap<String, serde_json::Value>> {
        let (rest, _) = ws(char('{'))(input)?;
        let (rest, pairs) = separated_list0(ws(char(',')), pair(ident, preceded(ws(char(':')), value_literal)))(rest)?;
        let (rest, _) = ws(char('}'))(rest)?;
        Ok((rest, pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()))
    }

    fn value_literal(input: &str) -> IResult<&str, serde_json::Value> {
        alt((
            literal_value,
            map(delimited(ws(char('[')), separated_list0(ws(char(',')), value_literal), ws(char(']'))), |v| {
                serde_json::Value::Array(v)
            }),
            map(object_literal, |m| serde_json::Value::Object(m.into_iter().collect())),
        ))(input)
    }

    fn primary_expr(input: &str) -> IResult<&str, Expr> {
        alt((
            map(delimited(ws(char('(')), cut(expr), ws(char(')'))), |e| e),
            map(literal_value, Expr::Const),
            map(
                separated_list1(ws(char('.')), ident),
                |parts: Vec<&str>| {
                    let mut it = parts.into_iter();
                    let first = it.next().unwrap_or("_");
                    let mut cur = Expr::Var {
                        name: first.to_string(),
                    };
                    for p in it {
                        cur = Expr::Field {
                            object: Box::new(cur),
                            name: p.to_string(),
                        };
                    }
                    cur
                },
            ),
        ))(input)
    }

    fn unary_expr(input: &str) -> IResult<&str, Expr> {
        alt((
            map(preceded(ws(char('-')), cut(unary_expr)), |e| Expr::UnaryNeg(Box::new(e))),
            primary_expr,
        ))(input)
    }

    fn mul_expr(input: &str) -> IResult<&str, Expr> {
        let (rest, first) = unary_expr(input)?;
        let (rest, tail) = many0(pair(
            ws(alt((char('*'), char('/')))),
            cut(unary_expr),
        ))(rest)?;
        let mut cur = first;
        for (op, rhs) in tail {
            cur = Expr::BinOp {
                op: if op == '*' { BinOp::Mul } else { BinOp::Div },
                left: Box::new(cur),
                right: Box::new(rhs),
            };
        }
        Ok((rest, cur))
    }

    fn add_expr(input: &str) -> IResult<&str, Expr> {
        let (rest, first) = mul_expr(input)?;
        let (rest, tail) = many0(pair(
            ws(alt((char('+'), char('-')))),
            cut(mul_expr),
        ))(rest)?;
        let mut cur = first;
        for (op, rhs) in tail {
            cur = Expr::BinOp {
                op: if op == '+' { BinOp::Add } else { BinOp::Sub },
                left: Box::new(cur),
                right: Box::new(rhs),
            };
        }
        Ok((rest, cur))
    }

    fn cmp_expr(input: &str) -> IResult<&str, Expr> {
        let (rest, left) = add_expr(input)?;
        let (rest, op_rhs) = opt(pair(
            ws(alt((
                tag("=="),
                tag("!="),
                tag("<="),
                tag(">="),
                tag("="),
                tag("<"),
                tag(">"),
            ))),
            cut(add_expr),
        ))(rest)?;
        if let Some((op, right)) = op_rhs {
            let bop = match op {
                "==" | "=" => BinOp::Eq,
                "!=" => BinOp::Ne,
                "<=" => BinOp::Le,
                ">=" => BinOp::Ge,
                "<" => BinOp::Lt,
                ">" => BinOp::Gt,
                _ => BinOp::Eq,
            };
            return Ok((
                rest,
                Expr::BinOp {
                    op: bop,
                    left: Box::new(left),
                    right: Box::new(right),
                },
            ));
        }
        Ok((rest, left))
    }

    fn and_expr(input: &str) -> IResult<&str, Expr> {
        let (rest, parts) = separated_list1(kw("AND"), cmp_expr)(input)?;
        if parts.len() == 1 {
            Ok((rest, parts.into_iter().next().unwrap()))
        } else {
            Ok((rest, Expr::And(parts)))
        }
    }

    fn or_expr(input: &str) -> IResult<&str, Expr> {
        let (rest, parts) = separated_list1(kw("OR"), and_expr)(input)?;
        if parts.len() == 1 {
            Ok((rest, parts.into_iter().next().unwrap()))
        } else {
            Ok((rest, Expr::Or(parts)))
        }
    }

    fn expr(input: &str) -> IResult<&str, Expr> {
        or_expr(input)
    }

    fn prop_map(input: &str) -> IResult<&str, BTreeMap<String, serde_json::Value>> {
        let (rest, _) = ws(char('{'))(input)?;
        let (rest, pairs) = separated_list0(ws(char(',')), pair(ident, preceded(ws(char(':')), literal_value)))(rest)?;
        let (rest, _) = ws(char('}'))(rest)?;
        Ok((rest, pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()))
    }

    fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
        let (rest, _) = ws(char('('))(input)?;
        let (rest, var_name) = opt(ident)(rest)?;
        let (rest, label) = opt(preceded(ws(char(':')), ident))(rest)?;
        let (rest, props) = opt(prop_map)(rest)?;
        let (rest, _) = ws(char(')'))(rest)?;
        Ok((
            rest,
            NodePattern {
                var_name: var_name.map(str::to_string),
                label: label.map(str::to_string),
                props: props.unwrap_or_default(),
            },
        ))
    }

    fn rel_types(input: &str) -> IResult<&str, (Vec<String>, BTreeMap<String, serde_json::Value>)> {
        let (rest, _) = ws(char('['))(input)?;
        let (rest, _) = opt(ws(char(':')))(rest)?;
        let (rest, first) = opt(ident)(rest)?;
        let (rest, more) = many0(preceded(ws(char('|')), ident))(rest)?;
        let (rest, props) = opt(prop_map)(rest)?;
        let (rest, _) = ws(char(']'))(rest)?;
        let mut types = Vec::new();
        if let Some(t) = first {
            types.push(t.to_string());
        }
        for t in more {
            types.push(t.to_string());
        }
        Ok((rest, (types, props.unwrap_or_default())))
    }

    fn rel_pattern(input: &str) -> IResult<&str, RelPattern> {
        let (rest, _) = ws(char('-'))(input)?;
        let (rest, inbound) = opt(ws(char('<')))(rest)?;
        let (rest, types_props) = rel_types(rest)?;
        let (rest, outbound) = opt(alt((
            map(pair(ws(char('-')), ws(char('>'))), |_| ()),
            map(ws(char('-')), |_| ()),
            map(ws(char('>')), |_| ()),
        )))(rest)?;
        let direction = if inbound.is_some() && outbound.is_some() {
            RelDirection::Undirected
        } else if inbound.is_some() {
            RelDirection::In
        } else if outbound.is_some() {
            RelDirection::Out
        } else {
            RelDirection::Undirected
        };
        Ok((
            rest,
            RelPattern {
                types: types_props.0,
                direction,
                props: types_props.1,
            },
        ))
    }

    fn pattern(input: &str) -> IResult<&str, Pattern> {
        let (rest, first) = node_pattern(input)?;
        let (rest, pairs) = many0(pair(rel_pattern, node_pattern))(rest)?;
        let mut elements = vec![PatternElement::Node(first)];
        for (rel, node) in pairs {
            elements.push(PatternElement::Rel(rel));
            elements.push(PatternElement::Node(node));
        }
        Ok((rest, Pattern { elements }))
    }

    fn pattern_list(input: &str) -> IResult<&str, Vec<Pattern>> {
        separated_list1(ws(char(',')), pattern)(input)
    }

    fn projection_list(input: &str) -> IResult<&str, Vec<ProjectionItem>> {
        separated_list1(
            ws(char(',')),
            map(
                pair(expr, opt(preceded(kw("AS"), cut(ident)))),
                |(e, alias)| ProjectionItem {
                    expr: e,
                    alias: alias.map(str::to_string),
                },
            ),
        )(input)
    }

    fn yield_clause(input: &str) -> IResult<&str, Vec<YieldItem>> {
        let (rest, _) = kw("YIELD")(input)?;
        separated_list1(
            ws(char(',')),
            map(
                pair(
                    separated_list1(ws(char('.')), ident),
                    opt(preceded(kw("AS"), cut(ident))),
                ),
                |(parts, alias)| YieldItem {
                    key: parts.join("."),
                    alias: alias.map(str::to_string),
                },
            ),
        )(rest)
    }

    fn match_clause(input: &str) -> IResult<&str, MatchClause> {
        let (rest, _) = kw("MATCH")(input)?;
        let (rest, patterns) = cut(pattern_list)(rest)?;
        let (rest, where_expr) = opt(preceded(kw("WHERE"), cut(expr)))(rest)?;
        Ok((
            rest,
            MatchClause {
                patterns,
                where_expr,
            },
        ))
    }

    fn with_clause(input: &str) -> IResult<&str, WithClause> {
        let (rest, _) = kw("WITH")(input)?;
        let (rest, projections) = cut(projection_list)(rest)?;
        let (rest, where_expr) = opt(preceded(kw("WHERE"), cut(expr)))(rest)?;
        Ok((
            rest,
            WithClause {
                projections,
                where_expr,
            },
        ))
    }

    fn call_clause(input: &str) -> IResult<&str, CallClause> {
        let (rest, _) = kw("CALL")(input)?;
        let (rest, parts) = cut(separated_list1(ws(char('.')), ident))(rest)?;
        let (rest, args) = delimited(ws(char('(')), opt(object_literal), ws(char(')')))(rest)?;
        let (rest, yield_items) = opt(yield_clause)(rest)?;
        Ok((
            rest,
            CallClause {
                action_id: parts.join("."),
                args: args.unwrap_or_default(),
                yield_items: yield_items.unwrap_or_default(),
            },
        ))
    }

    fn unwind_clause(input: &str) -> IResult<&str, UnwindClause> {
        let (rest, _) = kw("UNWIND")(input)?;
        let (rest, source) = cut(expr)(rest)?;
        let (rest, _) = kw("AS")(rest)?;
        let (rest, alias) = cut(ident)(rest)?;
        let (rest, where_expr) = opt(preceded(kw("WHERE"), cut(expr)))(rest)?;
        Ok((
            rest,
            UnwindClause {
                source,
                alias: alias.to_string(),
                where_expr,
            },
        ))
    }

    fn return_clause(input: &str) -> IResult<&str, ReturnClause> {
        let (rest, _) = kw("RETURN")(input)?;
        let (rest, projections) = cut(projection_list)(rest)?;
        let (rest, order_by) = opt(preceded(pair(kw("ORDER"), kw("BY")), cut(expr)))(rest)?;
        let (rest, limit) = opt(preceded(
            kw("LIMIT"),
            map(ws(digit1), |s: &str| s.parse::<usize>().unwrap_or(0)),
        ))(rest)?;
        Ok((
            rest,
            ReturnClause {
                projections,
                order_by,
                limit,
            },
        ))
    }

    fn clause(input: &str) -> IResult<&str, Clause> {
        alt((
            map(match_clause, Clause::Match),
            map(with_clause, Clause::With),
            map(call_clause, Clause::Call),
            map(unwind_clause, Clause::Unwind),
        ))(input)
    }

    fn query(input: &str) -> IResult<&str, Query> {
        let (rest, clauses) = many0(clause)(input)?;
        let (rest, return_clause) = opt(return_clause)(rest)?;
        let (rest, _) = multispace0(rest)?;
        Ok((
            rest,
            Query {
                clauses,
                return_clause,
            },
        ))
    }

    /// @emoji 🔍 Parse architect source into `Query`.
    pub fn parse_query(text: &str) -> Result<Query, ArchitectError> {
        match query(text) {
            Ok(("", ast)) => Ok(ast),
            Ok((rem, _)) => Err(ArchitectError::Parse(format!("trailing input: {rem:?}"))),
            Err(e) => Err(ArchitectError::Parse(format!("{e}"))),
        }
    }
}
//#endregion 🔖Parser

//#region 🔖Schema
mod schema {
    use super::ast::{NodePattern, RelPattern};
    use std::collections::BTreeMap;

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

    /// @emoji 🔗 Architect relationship predicate.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Predicate {
        Has,
        Is,
        References,
        Owns,
    }

    impl Predicate {
        pub fn parse(s: &str) -> Option<Self> {
            Some(match s.to_ascii_uppercase().as_str() {
                "HAS" => Self::Has,
                "IS" => Self::Is,
                "REFERENCES" => Self::References,
                "OWNS" => Self::Owns,
                _ => return None,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Cardinality {
        One,
        Many,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EdgeProp {
        Parent(bool),
    }

    #[derive(Debug, Clone, Copy)]
    pub struct EdgeDef {
        pub from: Label,
        pub pred: Predicate,
        pub to: Label,
        pub field: &'static str,
        pub cardinality: Cardinality,
        pub fragment: Option<&'static str>,
        pub edge_props: &'static [(&'static str, EdgeProp)],
    }

    pub const EDGES: &[EdgeDef] = &[
        EdgeDef {
            from: Label::Piece,
            pred: Predicate::Has,
            to: Label::Blueprint,
            field: "blueprint",
            cardinality: Cardinality::One,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Blueprint,
            pred: Predicate::Is,
            to: Label::Type,
            field: "__typename",
            cardinality: Cardinality::One,
            fragment: Some("... on Type"),
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Blueprint,
            pred: Predicate::Is,
            to: Label::Design,
            field: "__typename",
            cardinality: Cardinality::One,
            fragment: Some("... on Design"),
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Type,
            pred: Predicate::Has,
            to: Label::Connector,
            field: "connectors",
            cardinality: Cardinality::Many,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Type,
            pred: Predicate::Has,
            to: Label::Port,
            field: "ports",
            cardinality: Cardinality::Many,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Connector,
            pred: Predicate::Is,
            to: Label::Port,
            field: "port",
            cardinality: Cardinality::One,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Side,
            pred: Predicate::References,
            to: Label::Connector,
            field: "connector",
            cardinality: Cardinality::One,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Connection,
            pred: Predicate::Has,
            to: Label::Side,
            field: "parent",
            cardinality: Cardinality::One,
            fragment: None,
            edge_props: &[("parent", EdgeProp::Parent(true))],
        },
        EdgeDef {
            from: Label::Connection,
            pred: Predicate::Has,
            to: Label::Side,
            field: "child",
            cardinality: Cardinality::One,
            fragment: None,
            edge_props: &[("parent", EdgeProp::Parent(false))],
        },
        EdgeDef {
            from: Label::Design,
            pred: Predicate::Has,
            to: Label::Connection,
            field: "connections",
            cardinality: Cardinality::Many,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Design,
            pred: Predicate::Has,
            to: Label::Piece,
            field: "pieces",
            cardinality: Cardinality::Many,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Kit,
            pred: Predicate::Has,
            to: Label::Design,
            field: "designs",
            cardinality: Cardinality::Many,
            fragment: None,
            edge_props: &[],
        },
        EdgeDef {
            from: Label::Kit,
            pred: Predicate::Has,
            to: Label::Type,
            field: "types",
            cardinality: Cardinality::Many,
            fragment: None,
            edge_props: &[],
        },
    ];

    pub fn resolve_edge(
        from: Label,
        pred: Predicate,
        to: Label,
        rel: &RelPattern,
        forward: bool,
    ) -> Result<EdgeDef, String> {
        let mut matches: Vec<EdgeDef> = EDGES
            .iter()
            .copied()
            .filter(|e| e.from == from && e.pred == pred && e.to == to && edge_props_match(e, rel))
            .collect();
        if !forward {
            matches = EDGES
                .iter()
                .copied()
                .filter(|e| e.from == to && e.pred == pred && e.to == from && edge_props_match(e, rel))
                .collect();
        }
        match matches.len() {
            0 => Err(format!("no edge {from:?}-{pred:?}->{to:?}")),
            1 => Ok(matches[0]),
            _ => Err(format!("ambiguous edge {from:?}-{pred:?}->{to:?}")),
        }
    }

    fn edge_props_match(edge: &EdgeDef, rel: &RelPattern) -> bool {
        if rel.props.is_empty() {
            return edge.edge_props.is_empty()
                || edge.edge_props.iter().all(|(_, p)| matches!(p, EdgeProp::Parent(false)));
        }
        for (k, v) in &rel.props {
            if k == "parent" {
                let want = v.as_bool().unwrap_or(false);
                let ok = edge.edge_props.iter().any(|(name, p)| {
                    name == &"parent" && matches!(p, EdgeProp::Parent(g) if *g == want)
                });
                if !ok {
                    return false;
                }
            }
        }
        true
    }

    pub fn node_label(node: &NodePattern) -> Result<Label, String> {
        let lab = node
            .label
            .as_deref()
            .ok_or_else(|| "pattern node requires a label".to_string())?;
        Label::parse(lab).ok_or_else(|| format!("unknown label {lab}"))
    }

    pub fn rel_predicate(rel: &RelPattern) -> Result<Predicate, String> {
        let t = rel
            .types
            .first()
            .ok_or_else(|| "relationship requires a predicate".to_string())?;
        Predicate::parse(t).ok_or_else(|| format!("unknown predicate {t}"))
    }

    pub fn node_filter_field(label: Label, key: &str) -> &str {
        match (label, key) {
            (Label::Port, "name") => "label",
            (Label::Design, "name") => "name",
            (Label::Type, "name") => "name",
            (Label::Kit, "name") => "name",
            (Label::Piece, "name") => "name",
            (Label::Connector, "name") => "name",
            _ => key,
        }
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

    /// @emoji 📞 Static `CALL` target (mutation or subscription).
    #[derive(Debug, Clone, Copy)]
    pub struct CallTarget {
        pub path: &'static [&'static str],
        pub kind: super::transport::OpKind,
        pub gql: &'static str,
    }

    pub const CALL_TARGETS: &[CallTarget] = &[
        CallTarget {
            path: &["session", "start"],
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall($input: String) { session { start } }",
        },
        CallTarget {
            path: &["session", "end"],
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall { session { end { ok errors { message } } } }",
        },
        CallTarget {
            path: &["subscription", "session"],
            kind: super::transport::OpKind::Subscription,
            gql: "subscription ArchitectSub { session { id hash } }",
        },
        CallTarget {
            path: &["subscription", "operation"],
            kind: super::transport::OpKind::Subscription,
            gql: "subscription ArchitectSub { operation { id hash } }",
        },
        CallTarget {
            path: &["session", "store", "installProjection"],
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall($storeId: ID!, $json: String!) { session { store(id: $storeId) { installProjection(json: $json) { ok errors { message } } } } }",
        },
        CallTarget {
            path: &["session", "store", "theKit", "startNewChange"],
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange { ok errors { message } } } } } }",
        },
        CallTarget {
            path: &["session", "store", "theKit", "save"],
            kind: super::transport::OpKind::Mutation,
            gql: "mutation ArchitectCall($storeId: ID!) { session { store(id: $storeId) { theKit { save { ok errors { message } } } } } }",
        },
    ];

    pub fn resolve_call(action_id: &str) -> Result<CallTarget, String> {
        let parts: Vec<&str> = action_id.split('.').collect();
        CALL_TARGETS
            .iter()
            .copied()
            .find(|t| t.path.len() == parts.len() && t.path.iter().zip(&parts).all(|(a, b)| a == b))
            .ok_or_else(|| format!("unknown CALL target {action_id}"))
    }

    pub fn call_variables(action_id: &str, args: &BTreeMap<String, serde_json::Value>) -> serde_json::Value {
        let mut vars = serde_json::Map::new();
        if action_id.starts_with("session.store") {
            if let Some(v) = args.get("store").or_else(|| args.get("storeId")) {
                vars.insert("storeId".into(), v.clone());
            }
            if let Some(v) = args.get("json") {
                vars.insert("json".into(), v.clone());
            }
        }
        serde_json::Value::Object(vars)
    }
}
//#endregion 🔖Schema

//#region 🔖Transport
mod transport {
    use futures_util::stream::{self, BoxStream};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use thiserror::Error;

    /// @emoji 📡 GraphQL operation kind for the host transport.
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

    /// @emoji 🌐 Async GraphQL IO boundary (native + wasm).
    pub trait Transport {
        fn execute(
            &self,
            kind: OpKind,
            doc: &str,
            variables: Value,
        ) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>>;

        fn subscribe(
            &self,
            doc: &str,
            variables: Value,
        ) -> Pin<Box<dyn Future<Output = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>> + '_>>;
    }

    /// @emoji 🧪 In-memory transport for unit tests.
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
        fn execute(
            &self,
            kind: OpKind,
            doc: &str,
            variables: Value,
        ) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>> {
            let key = Self::key(kind, doc);
            let _ = variables;
            let out = self
                .responses
                .get(&key)
                .or_else(|| self.responses.get(doc))
                .cloned()
                .ok_or_else(|| TransportError::Msg(format!("no canned response for {key}")));
            Box::pin(async move { out })
        }

        fn subscribe(
            &self,
            doc: &str,
            variables: Value,
        ) -> Pin<Box<dyn Future<Output = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>> + '_>> {
            let key = Self::key(OpKind::Subscription, doc);
            let _ = variables;
            let item = self
                .responses
                .get(&key)
                .or_else(|| self.responses.get(doc))
                .cloned()
                .ok_or_else(|| TransportError::Msg(format!("no canned subscription for {key}")));
            Box::pin(async move {
                Ok(Box::pin(stream::once(async move { item })) as BoxStream<'static, Result<Value, TransportError>>)
            })
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
            Self {
                execute_fn,
                subscribe_fn,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    impl Transport for JsTransport {
        fn execute(
            &self,
            kind: OpKind,
            doc: &str,
            variables: Value,
        ) -> Pin<Box<dyn Future<Output = Result<Value, TransportError>> + '_>> {
            let execute_fn = self.execute_fn.clone();
            let doc = doc.to_string();
            let kind_s = format!("{kind:?}");
            Box::pin(async move {
                let vars = serde_wasm_bindgen::to_value(&variables)
                    .map_err(|e| TransportError::Msg(e.to_string()))?;
                let promise = execute_fn
                    .call2(
                        &wasm_bindgen::JsValue::NULL,
                        &wasm_bindgen::JsValue::from_str(&kind_s),
                        &wasm_bindgen::JsValue::from_str(&doc),
                    )
                    .map_err(|e| TransportError::Msg(format!("{e:?}")))?;
                let _ = vars;
                let val = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&promise))
                    .await
                    .map_err(|e| TransportError::Msg(format!("{e:?}")))?;
                serde_wasm_bindgen::from_value(val).map_err(|e| TransportError::Msg(e.to_string()))
            })
        }

        fn subscribe(
            &self,
            doc: &str,
            variables: Value,
        ) -> Pin<Box<dyn Future<Output = Result<BoxStream<'static, Result<Value, TransportError>>, TransportError>> + '_>> {
            let subscribe_fn = self.subscribe_fn.clone();
            let doc = doc.to_string();
            let vars = variables;
            Box::pin(async move {
                let _vars = vars;
                let _stream_factory = subscribe_fn
                    .call1(
                        &wasm_bindgen::JsValue::NULL,
                        &wasm_bindgen::JsValue::from_str(&doc),
                    )
                    .map_err(|e| TransportError::Msg(format!("{e:?}")))?;
                Err(TransportError::Msg(
                    "JsTransport subscription stream wiring is host-specific".into(),
                ))
            })
        }
    }

    /// @emoji 🔁 Shared canned responses for parallel test runs.
    pub type SharedMemoryTransport = Arc<Mutex<MemoryTransport>>;
}
//#endregion 🔖Transport

//#region 🔖Planner
mod planner {
    use super::ast::*;
    use super::errors::ArchitectError;
    use super::schema::{self, Label};
    use super::transport::OpKind;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, BTreeSet};

    /// @emoji 🧭 Planned execution steps for an architect query.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct OpPlan {
        pub steps: Vec<Step>,
        pub return_clause: Option<ReturnClause>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum Step {
        GraphQl {
            op: OpKind,
            document: String,
            variables: Value,
            bind: BindSpec,
        },
        Join {
            on_var: String,
            key: String,
        },
        Filter {
            expr: Expr,
        },
        Unwind {
            source_var: String,
            alias: String,
            where_expr: Option<Expr>,
        },
        Project {
            projections: Vec<ProjectionItem>,
            where_expr: Option<Expr>,
        },
        Order {
            expr: Expr,
        },
        Limit {
            n: usize,
        },
        Call {
            op: OpKind,
            document: String,
            variables: Value,
            yield_items: Vec<YieldItem>,
        },
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct BindSpec {
        pub anchor_var: String,
        pub anchor_label: String,
        pub paths: BTreeMap<String, JsonPath>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct JsonPath {
        pub segments: Vec<PathSeg>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum PathSeg {
        Field { name: String },
        ConnectionEdges,
        ConnectionNode,
        Fragment { name: String },
    }

    struct PatternPlan {
        anchor_var: String,
        anchor_label: Label,
        document: String,
        bind: BindSpec,
    }

    /// @emoji 🧭 Lower `Query` AST to `OpPlan`.
    pub fn plan_query(q: &Query) -> Result<OpPlan, ArchitectError> {
        let mut steps = Vec::new();
        let mut emitted_patterns: BTreeSet<String> = BTreeSet::new();
        let mut join_vars: BTreeSet<String> = BTreeSet::new();

        for clause in &q.clauses {
            match clause {
                Clause::Match(m) => {
                    let mut shared: BTreeMap<String, usize> = BTreeMap::new();
                    for pat in &m.patterns {
                        for el in &pat.elements {
                            if let PatternElement::Node(n) = el {
                                if let Some(v) = &n.var_name {
                                    *shared.entry(v.clone()).or_default() += 1;
                                }
                            }
                        }
                    }
                    for pat in &m.patterns {
                        let plan = plan_pattern(pat)?;
                        if emitted_patterns.insert(plan.document.clone()) {
                            steps.push(Step::GraphQl {
                                op: OpKind::Query,
                                document: plan.document,
                                variables: json!({}),
                                bind: plan.bind,
                            });
                        }
                    }
                    for (var, count) in &shared {
                        if *count > 1 && join_vars.insert(var.clone()) {
                            steps.push(Step::Join {
                                on_var: var.clone(),
                                key: "id".into(),
                            });
                        }
                    }
                    if let Some(w) = &m.where_expr {
                        steps.push(Step::Filter { expr: w.clone() });
                    }
                }
                Clause::With(w) => {
                    steps.push(Step::Project {
                        projections: w.projections.clone(),
                        where_expr: w.where_expr.clone(),
                    });
                }
                Clause::Unwind(u) => {
                    let source_var = match &u.source {
                        Expr::Var { name } => name.clone(),
                        other => {
                            return Err(ArchitectError::Plan(format!(
                                "UNWIND expects a variable, got {other:?}"
                            )));
                        }
                    };
                    steps.push(Step::Unwind {
                        source_var,
                        alias: u.alias.clone(),
                        where_expr: u.where_expr.clone(),
                    });
                }
                Clause::Call(c) => {
                    let target = schema::resolve_call(&c.action_id)
                        .map_err(ArchitectError::Plan)?;
                    steps.push(Step::Call {
                        op: target.kind,
                        document: target.gql.to_string(),
                        variables: schema::call_variables(&c.action_id, &c.args),
                        yield_items: c.yield_items.clone(),
                    });
                }
            }
        }

        if let Some(ret) = &q.return_clause {
            if ret.order_by.is_some() {
                steps.push(Step::Order {
                    expr: ret.order_by.clone().unwrap(),
                });
            }
            if let Some(n) = ret.limit {
                steps.push(Step::Limit { n });
            }
        }

        Ok(OpPlan {
            steps,
            return_clause: q.return_clause.clone(),
        })
    }

    fn plan_pattern(pat: &Pattern) -> Result<PatternPlan, ArchitectError> {
        let nodes: Vec<&NodePattern> = pat
            .elements
            .iter()
            .filter_map(|e| match e {
                PatternElement::Node(n) => Some(n),
                _ => None,
            })
            .collect();
        if nodes.is_empty() {
            return Err(ArchitectError::Plan("empty pattern".into()));
        }
        let anchor = nodes
            .iter()
            .min_by_key(|n| selectivity(n))
            .ok_or_else(|| ArchitectError::Plan("no anchor".into()))?;
        let anchor_var = anchor
            .var_name
            .clone()
            .unwrap_or_else(|| "__anchor".into());
        let anchor_label = schema::node_label(anchor).map_err(ArchitectError::Plan)?;

        let (document, paths) = build_graphql_document(pat, anchor_var.as_str(), anchor_label)?;
        let bind = BindSpec {
            anchor_var: anchor_var.clone(),
            anchor_label: anchor_label.gql_name().to_string(),
            paths,
        };
        Ok(PatternPlan {
            anchor_var,
            anchor_label,
            document,
            bind,
        })
    }

    fn selectivity(n: &NodePattern) -> u8 {
        let mut score = 10u8;
        if !n.props.is_empty() {
            score = score.saturating_sub(5);
        }
        if n.label.is_some() {
            score = score.saturating_sub(2);
        }
        score
    }

    fn build_graphql_document(
        pat: &Pattern,
        anchor_var: &str,
        anchor_label: Label,
    ) -> Result<(String, BTreeMap<String, JsonPath>), ArchitectError> {
        let mut paths: BTreeMap<String, JsonPath> = BTreeMap::new();
        let mut anchor_path = vec![
            PathSeg::Field {
                name: "session".into(),
            },
            PathSeg::Field {
                name: "stores".into(),
            },
            PathSeg::ConnectionEdges,
            PathSeg::ConnectionNode,
            PathSeg::Field {
                name: "wip".into(),
            },
            PathSeg::Field {
                name: "theKit".into(),
            },
            PathSeg::Field {
                name: "kit".into(),
            },
        ];

        match anchor_label {
            Label::Design => {
                anchor_path.push(PathSeg::Field {
                    name: "designs".into(),
                });
                anchor_path.push(PathSeg::ConnectionEdges);
                anchor_path.push(PathSeg::ConnectionNode);
            }
            Label::Type => {
                anchor_path.push(PathSeg::Field {
                    name: "types".into(),
                });
                anchor_path.push(PathSeg::ConnectionEdges);
                anchor_path.push(PathSeg::ConnectionNode);
            }
            Label::Kit => {}
            _ => {
                return Err(ArchitectError::Plan(format!(
                    "anchor label {} must be Kit, Design, or Type for session root",
                    anchor_label.gql_name()
                )));
            }
        }

        paths.insert(
            anchor_var.to_string(),
            JsonPath {
                segments: anchor_path.clone(),
            },
        );

        let mut cursor_label = anchor_label;
        let mut cursor_path = anchor_path;
        let mut node_iter = pat.elements.iter().peekable();
        while let Some(el) = node_iter.next() {
            let PatternElement::Node(node) = el else { continue };
            if let Some(PatternElement::Rel(rel)) = node_iter.peek() {
                let next_node = loop {
                    node_iter.next();
                    if let Some(PatternElement::Node(n)) = node_iter.peek() {
                        break n;
                    }
                    node_iter.next();
                };
                let to_label = schema::node_label(next_node).map_err(ArchitectError::Plan)?;
                let pred = schema::rel_predicate(rel).map_err(ArchitectError::Plan)?;
                let forward = match rel.direction {
                    RelDirection::Out | RelDirection::Undirected => true,
                    RelDirection::In => false,
                };
                let edge = schema::resolve_edge(cursor_label, pred, to_label, rel, forward)
                    .map_err(ArchitectError::Plan)?;
                cursor_path.push(PathSeg::Field {
                    name: edge.field.to_string(),
                });
                if edge.cardinality == schema::Cardinality::Many {
                    cursor_path.push(PathSeg::ConnectionEdges);
                    cursor_path.push(PathSeg::ConnectionNode);
                }
                if let Some(frag) = edge.fragment {
                    cursor_path.push(PathSeg::Fragment {
                        name: frag.to_string(),
                    });
                }
                if let Some(v) = &next_node.var_name {
                    paths.insert(
                        v.clone(),
                        JsonPath {
                            segments: cursor_path.clone(),
                        },
                    );
                }
                cursor_label = to_label;
            } else if node.var_name.as_deref() != Some(anchor_var) {
                if let Some(v) = &node.var_name {
                    paths.insert(
                        v.clone(),
                        JsonPath {
                            segments: cursor_path.clone(),
                        },
                    );
                }
            }
        }

        let mut body = String::from("query ArchitectMatch {\n  session {\n    stores {\n      edges {\n        node {\n          wip {\n            theKit {\n              kit {\n");
        match anchor_label {
            Label::Design => {
                body.push_str("                designs {\n                  edges {\n                    node {\n");
                body.push_str(&build_nested_selection(pat, anchor_label, 1));
                body.push_str("                    }\n                  }\n                }\n");
            }
            Label::Type => {
                body.push_str("                types {\n                  edges {\n                    node {\n");
                body.push_str(&build_nested_selection(pat, anchor_label, 1));
                body.push_str("                    }\n                  }\n                }\n");
            }
            Label::Kit => {
                body.push_str(&build_nested_selection(pat, anchor_label, 0));
            }
            _ => {}
        }
        body.push_str("              }\n            }\n          }\n        }\n      }\n    }\n  }\n}\n");
        Ok((body, paths))
    }

    fn build_nested_selection(pat: &Pattern, anchor_label: Label, start_idx: usize) -> String {
        let mut out = String::new();
        let scalars = schema::entity_scalar_fields(anchor_label).join(" ");
        out.push_str(&format!("                      {scalars}\n"));
        let elements: Vec<_> = pat.elements.iter().collect();
        let mut i = start_idx;
        while i < elements.len() {
            if let PatternElement::Node(node) = elements[i] {
                if i + 1 < elements.len() {
                    if let PatternElement::Rel(rel) = elements[i + 1] {
                        if i + 2 < elements.len() {
                            if let PatternElement::Node(next) = elements[i + 2] {
                                if let (Some(from), Some(to)) = (
                                    node.label.as_ref().and_then(|l| Label::parse(l)),
                                    next.label.as_ref().and_then(|l| Label::parse(l)),
                                ) {
                                    if let Ok(pred) = schema::rel_predicate(rel) {
                                        let forward = !matches!(rel.direction, RelDirection::In);
                                        if let Ok(edge) =
                                            schema::resolve_edge(from, pred, to, rel, forward)
                                        {
                                            if edge.field == "__typename" {
                                                if to == Label::Type {
                                                    out.push_str("                      ... on Type { id hash name connectors { edges { node { id hash name port { id hash label code } } } } }\n");
                                                } else if to == Label::Design {
                                                    out.push_str("                      ... on Design { id hash name }\n");
                                                }
                                            } else if edge.cardinality == schema::Cardinality::Many {
                                                let child_scalars =
                                                    schema::entity_scalar_fields(to).join(" ");
                                                out.push_str(&format!(
                                                    "                      {} {{ edges {{ node {{ {child_scalars} {} }} }} }}\n",
                                                    edge.field,
                                                    build_rel_tail(&elements, i + 2, to)
                                                ));
                                            } else {
                                                let child_scalars =
                                                    schema::entity_scalar_fields(to).join(" ");
                                                out.push_str(&format!(
                                                    "                      {} {{ {child_scalars} {} }}\n",
                                                    edge.field,
                                                    build_rel_tail(&elements, i + 2, to)
                                                ));
                                            }
                                        }
                                    }
                                }
                                i += 2;
                                continue;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        out
    }

    fn build_rel_tail(elements: &[&PatternElement], from_idx: usize, from_label: Label) -> String {
        let mut s = String::new();
        let mut i = from_idx + 1;
        while i < elements.len() {
            if let PatternElement::Rel(rel) = elements[i] {
                if i + 1 < elements.len() {
                    if let PatternElement::Node(next) = elements[i + 1] {
                        if let (Ok(pred), Some(to_lab)) = (
                            schema::rel_predicate(rel),
                            next.label.as_ref().and_then(|l| Label::parse(l)),
                        ) {
                            let forward = !matches!(rel.direction, RelDirection::In);
                            if let Ok(edge) =
                                schema::resolve_edge(from_label, pred, to_lab, rel, forward)
                            {
                                let child_scalars = schema::entity_scalar_fields(to_lab).join(" ");
                                if edge.cardinality == schema::Cardinality::Many {
                                    s.push_str(&format!(
                                        "{} {{ edges {{ node {{ {child_scalars} }} }} }} ",
                                        edge.field
                                    ));
                                } else {
                                    s.push_str(&format!("{} {{ {child_scalars} }} ", edge.field));
                                }
                            }
                        }
                        i += 2;
                        continue;
                    }
                }
            }
            i += 1;
        }
        s
    }
}
//#endregion 🔖Planner

//#region 🔖Executor
mod executor {
    use super::ast::*;
    use super::errors::ArchitectError;
    use super::planner::{JsonPath, OpPlan, PathSeg, Step};
    use super::schema::{self, Label};
    use super::transport::{OpKind, Transport};
    use futures_util::{stream, StreamExt};
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

    /// @emoji ⚙️ Runs `OpPlan` against a `Transport`.
    pub struct Executor;

    impl Executor {
        pub async fn run(plan: &OpPlan, transport: &dyn Transport) -> Result<QueryResult, ArchitectError> {
            let mut env = BindEnv::default();
            for step in &plan.steps {
                env.apply(step, transport).await?;
            }
            env.finish(plan.return_clause.as_ref())
        }

        pub async fn run_subscription(
            plan: &OpPlan,
            transport: &dyn Transport,
        ) -> Result<
            Pin<Box<dyn futures_util::Stream<Item = Result<QueryResult, ArchitectError>> + Send>>,
            ArchitectError,
        > {
            let has_sub = plan.steps.iter().any(|s| matches!(s, Step::Call { op: OpKind::Subscription, .. }));
            if !has_sub {
                return Err(ArchitectError::Execute("plan has no subscription CALL".into()));
            }
            let mut env = BindEnv::default();
            for step in &plan.steps {
                match step {
                    Step::Call {
                        op: OpKind::Subscription,
                        document,
                        variables,
                        yield_items,
                    } => {
                        let mut sub_stream = transport.subscribe(document, variables.clone()).await?;
                        let yield_items = yield_items.clone();
                        let ret = plan.return_clause.clone();
                        let first = sub_stream
                            .next()
                            .await
                            .ok_or_else(|| ArchitectError::Execute("empty subscription stream".into()))??;
                        env.rows.clear();
                        env.ingest_call_yield(&first, &yield_items);
                        let once = stream::once(async move { env.finish(ret.as_ref()) });
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
                Step::GraphQl {
                    op,
                    document,
                    variables,
                    bind,
                } => {
                    let data = transport.execute(*op, document, variables.clone()).await?;
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
                Step::Unwind {
                    source_var,
                    alias,
                    where_expr,
                } => {
                    let mut next = Vec::new();
                    for row in &self.rows {
                        let Some(v) = row.get(source_var) else { continue };
                        let items = v.as_array().cloned().unwrap_or_else(|| vec![v.clone()]);
                        for item in items {
                            let mut r = row.clone();
                            r.insert(alias.clone(), item);
                            if let Some(w) = where_expr {
                                if !eval_bool(w, &r) {
                                    continue;
                                }
                            }
                            next.push(r);
                        }
                    }
                    self.rows = next;
                }
                Step::Project {
                    projections,
                    where_expr,
                } => {
                    let mut next = Vec::new();
                    for row in &self.rows {
                        let mut r = Row::new();
                        for p in projections {
                            let v = eval_expr(&p.expr, row)?;
                            let key = p.alias.clone().unwrap_or_else(|| expr_key(&p.expr));
                            r.insert(key, v);
                        }
                        if let Some(w) = where_expr {
                            if !eval_bool(w, &r) {
                                continue;
                            }
                        }
                        next.push(r);
                    }
                    self.rows = next;
                }
                Step::Order { expr } => {
                    self.rows.sort_by(|a, b| {
                        let av = eval_expr(expr, a).ok().map(sort_key);
                        let bv = eval_expr(expr, b).ok().map(sort_key);
                        av.cmp(&bv)
                    });
                }
                Step::Limit { n } => {
                    self.rows.truncate(*n);
                }
                Step::Call {
                    op,
                    document,
                    variables,
                    yield_items,
                } => {
                    if *op == OpKind::Subscription {
                        return Ok(());
                    }
                    let data = transport.execute(*op, document, variables.clone()).await?;
                    self.ingest_call_yield(&data, yield_items);
                }
            }
            Ok(())
        }

        fn ingest_call_yield(&mut self, data: &Value, yield_items: &[YieldItem]) {
            let mut row = Row::new();
            if yield_items.is_empty() {
                row.insert("result".into(), data.clone());
            } else {
                for y in yield_items {
                    let key = y.alias.clone().unwrap_or_else(|| y.key.clone());
                    let val = resolve_yield(data, &y.key);
                    row.insert(key, val);
                }
            }
            self.rows = vec![row];
        }

        fn finish(&self, ret: Option<&ReturnClause>) -> Result<QueryResult, ArchitectError> {
            let Some(ret) = ret else {
                return Ok(QueryResult {
                    columns: vec![],
                    rows: self
                        .rows
                        .iter()
                        .map(|r| Value::Object(r.iter().map(|(k, v)| (k.clone(), v.clone())).collect()))
                        .collect(),
                });
            };
            let mut columns = Vec::new();
            let mut rows = Vec::new();
            for row in &self.rows {
                let mut out = Row::new();
                for p in &ret.projections {
                    let v = eval_expr(&p.expr, row)?;
                    let key = p.alias.clone().unwrap_or_else(|| expr_key(&p.expr));
                    if !columns.contains(&key) {
                        columns.push(key.clone());
                    }
                    out.insert(key, v);
                }
                rows.push(Value::Object(out.into_iter().collect()));
            }
            Ok(QueryResult { columns, rows })
        }
    }

    fn extract_rows(data: &Value, bind: &super::planner::BindSpec) -> Result<Vec<Row>, ArchitectError> {
        let root = data.get("data").unwrap_or(data);
        let anchor_path = bind
            .paths
            .get(&bind.anchor_var)
            .ok_or_else(|| ArchitectError::Execute("bind missing anchor path".into()))?;
        let anchors = read_path(root, anchor_path);
        if anchors.is_empty() {
            return Ok(vec![]);
        }
        let mut rows = Vec::new();
        for anchor in anchors {
            let mut row = Row::new();
            row.insert(bind.anchor_var.clone(), anchor.clone());
            for (var, _path) in &bind.paths {
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
                    PathSeg::Fragment { name } => {
                        let _ = name;
                        next.push(v.clone());
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

    fn resolve_yield(data: &Value, key: &str) -> Value {
        let root = data.get("data").unwrap_or(data);
        let mut cur = root.clone();
        for part in key.split('.') {
            cur = cur.get(part).cloned().unwrap_or(Value::Null);
        }
        cur
    }

    fn expr_key(e: &Expr) -> String {
        match e {
            Expr::Var { name } => name.clone(),
            Expr::Field { object, name } => format!("{}.{}", expr_key(object), name),
            _ => "_".into(),
        }
    }

    fn eval_bool(expr: &Expr, row: &Row) -> bool {
        match eval_expr(expr, row) {
            Ok(Value::Bool(b)) => b,
            Ok(Value::Null) => false,
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn sort_key(v: Value) -> String {
        match v {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            other => other.to_string(),
        }
    }

    fn eval_expr(expr: &Expr, row: &Row) -> Result<Value, ArchitectError> {
        Ok(match expr {
            Expr::Const(v) => v.clone(),
            Expr::Var { name } => row.get(name).cloned().unwrap_or(Value::Null),
            Expr::Field { object, name } => {
                let base = eval_expr(object, row)?;
                base.get(name).cloned().unwrap_or(Value::Null)
            }
            Expr::UnaryNeg(inner) => {
                let v = eval_expr(inner, row)?;
                json_num(-json_as_f64(&v)?)
            }
            Expr::BinOp { op, left, right } => {
                let l = eval_expr(left, row)?;
                let r = eval_expr(right, row)?;
                match op {
                    BinOp::Eq => Value::Bool(json_eq(&l, &r)),
                    BinOp::Ne => Value::Bool(!json_eq(&l, &r)),
                    BinOp::Lt => Value::Bool(json_as_f64(&l)? < json_as_f64(&r)?),
                    BinOp::Le => Value::Bool(json_as_f64(&l)? <= json_as_f64(&r)?),
                    BinOp::Gt => Value::Bool(json_as_f64(&l)? > json_as_f64(&r)?),
                    BinOp::Ge => Value::Bool(json_as_f64(&l)? >= json_as_f64(&r)?),
                    BinOp::Add => json_num(json_as_f64(&l)? + json_as_f64(&r)?),
                    BinOp::Sub => json_num(json_as_f64(&l)? - json_as_f64(&r)?),
                    BinOp::Mul => json_num(json_as_f64(&l)? * json_as_f64(&r)?),
                    BinOp::Div => json_num(json_as_f64(&l)? / json_as_f64(&r)?),
                }
            }
            Expr::And(xs) => Value::Bool(xs.iter().all(|x| eval_expr(x, row).ok().and_then(|v| v.as_bool()).unwrap_or(false))),
            Expr::Or(xs) => Value::Bool(xs.iter().any(|x| eval_expr(x, row).ok().and_then(|v| v.as_bool()).unwrap_or(false))),
        })
    }

    fn json_eq(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::String(x), Value::String(y)) => x == y,
            (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
            _ => a == b,
        }
    }

    fn json_as_f64(v: &Value) -> Result<f64, ArchitectError> {
        match v {
            Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0)),
            Value::String(s) => s.parse().map_err(|_| ArchitectError::Execute(format!("not numeric: {s}"))),
            _ => Ok(0.0),
        }
    }

    fn json_num(n: f64) -> Value {
        serde_json::Number::from_f64(n)
            .map(Value::Number)
            .unwrap_or(Value::Null)
    }

}
//#endregion 🔖Executor

//#region 🔖Api
mod api {
    use super::ast::Query;
    use super::errors::ArchitectError;
    use super::executor::{Executor, QueryResult};
    use super::parser::parse_query;
    use super::planner::{plan_query, OpPlan};
    use super::transport::Transport;
    use futures_util::StreamExt;

    /// @emoji 🔍 Parse architect source.
    pub fn parse(text: &str) -> Result<Query, ArchitectError> {
        parse_query(text)
    }

    /// @emoji 🧭 Plan architect AST.
    pub fn plan(ast: &Query) -> Result<OpPlan, ArchitectError> {
        plan_query(ast)
    }

    /// @emoji 📜 Compile to `OpPlan` JSON-friendly plan.
    pub fn compile(text: &str) -> Result<OpPlan, ArchitectError> {
        plan(&parse(text)?)
    }

    /// @emoji ▶️ Parse, plan, and execute end-to-end.
    pub async fn run(text: &str, transport: &dyn Transport) -> Result<QueryResult, ArchitectError> {
        let ast = parse(text)?;
        let plan = plan(&ast)?;
        if plan.steps.iter().any(|s| {
            matches!(
                s,
                super::planner::Step::Call {
                    op: super::transport::OpKind::Subscription,
                    ..
                }
            )
        }) {
            let mut stream = Executor::run_subscription(&plan, transport).await?;
            if let Some(first) = stream.next().await {
                return first;
            }
            return Err(ArchitectError::Execute("empty subscription stream".into()));
        }
        Executor::run(&plan, transport).await
    }

}
//#endregion 🔖Api

#[cfg(target_arch = "wasm32")]
mod wasm_api {
    use super::api;
    use super::transport::{JsTransport, Transport};
    use wasm_bindgen::prelude::*;

    /// @emoji 🌐 Compile architect query to JSON plan (wasm).
    #[wasm_bindgen(js_name = architectCompile)]
    pub fn architect_compile(query: &str) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();
        api::compile(query)
            .map(|p| serde_wasm_bindgen::to_value(&p).unwrap())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// @emoji 🌐 Run architect query via JS transport callbacks (wasm).
    #[wasm_bindgen(js_name = architectRun)]
    pub async fn architect_run(query: &str, execute_fn: js_sys::Function, subscribe_fn: js_sys::Function) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();
        let transport = JsTransport::new(execute_fn, subscribe_fn);
        api::run(query, &transport)
            .await
            .map(|r| serde_wasm_bindgen::to_value(&r).unwrap())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    const EXAMPLE: &str = r#"
MATCH
  (pi:Piece) -[:HAS]-> (b:Blueprint) -[:IS]-> (t:Type) -[:HAS]-> (cr:Connector) -[:IS]-> (po:Port {name: ''}),
  (cr) <-[:REFERENCES] (s:Side) <-[:HAS {parent: true}] (cn:Connection) <-[:HAS] (d:Design {name: 'Nakagin Capsule Tower'})
RETURN pi
"#;

    fn fixture_response() -> serde_json::Value {
        json!({
          "data": {
            "session": {
              "stores": {
                "edges": [{
                  "node": {
                    "wip": {
                      "theKit": {
                        "kit": {
                          "designs": {
                            "edges": [{
                              "node": {
                                "id": "design-1",
                                "hash": "dh1",
                                "name": "Nakagin Capsule Tower",
                                "connections": {
                                  "edges": [{
                                    "node": {
                                      "id": "conn-1",
                                      "hash": "ch1",
                                      "name": "c1",
                                      "parent": {
                                        "id": "side-1",
                                        "hash": "sh1",
                                        "connector": { "id": "cr-1", "hash": "crh1", "name": "C1" }
                                      }
                                    }
                                  }]
                                }
                              }
                            }]
                          }
                        }
                      }
                    }
                  }
                }]
              }
            }
          }
        })
    }

    #[test]
    fn parse_example() {
        let q = parse(EXAMPLE).expect("parse");
        assert!(matches!(q.clauses.first(), Some(ast::Clause::Match(_))));
        assert!(q.return_clause.is_some());
    }

    #[test]
    fn plan_example() {
        let q = parse(EXAMPLE).unwrap();
        let plan = plan(&q).unwrap();
        assert!(plan.steps.iter().any(|s| matches!(s, planner::Step::GraphQl { .. })));
        assert!(plan.steps.iter().any(|s| matches!(s, planner::Step::Join { on_var, .. } if on_var == "cr")));
    }

    #[tokio::test]
    async fn run_design_match() {
        let q = "MATCH (d:Design {name: 'Nakagin Capsule Tower'}) RETURN d.name AS name";
        let compiled = compile(q).unwrap();
        let doc = compiled
            .steps
            .iter()
            .find_map(|s| match s {
                planner::Step::GraphQl { document, .. } => Some(document.clone()),
                _ => None,
            })
            .expect("graphql step");
        let mut responses = HashMap::new();
        responses.insert(format!("Query:{doc}"), fixture_response());
        let transport = MemoryTransport::new(responses);
        let result = run(q, &transport).await.unwrap();
        assert_eq!(result.columns, vec!["name"]);
        assert_eq!(result.rows.len(), 1);
    }

    #[test]
    fn call_mutation_plan() {
        let q = parse("CALL session.end() YIELD ok").unwrap();
        let p = plan(&q).unwrap();
        assert!(p.steps.iter().any(|s| matches!(
            s,
            planner::Step::Call {
                op: transport::OpKind::Mutation,
                ..
            }
        )));
    }

    #[tokio::test]
    async fn call_subscription_run() {
        let q = "CALL subscription.session() YIELD session.id AS id";
        let p = compile(q).unwrap();
        let doc = p
            .steps
            .iter()
            .find_map(|s| match s {
                planner::Step::Call { document, .. } => Some(document.clone()),
                _ => None,
            })
            .unwrap();
        let mut responses = HashMap::new();
        responses.insert(
            format!("Subscription:{doc}"),
            json!({ "data": { "session": { "id": "s1", "hash": "h1" } } }),
        );
        let transport = MemoryTransport::new(responses);
        let result = run(q, &transport).await.unwrap();
        assert!(!result.rows.is_empty());
    }
}
