//! Scratch verification crate for the F3b svg diff/mutations rewrite. Verbatim-ish port of the
//! real `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` bodies, with derive-macro /
//! store-crate / OpText/OpBinary plumbing stripped and a tiny local `protocol` trait shim,
//! isolating the algorithm from the real crate so it can be verified while the real crate's
//! unrelated jpg/tiff artifacts are mid-edit by other concurrent sessions (workspace churn).
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

//#region protocol shim
mod protocol {
    pub trait MutationDiff<P>: Clone + Default {
        fn apply(&self, base: &P) -> P;
        fn absorb(&mut self, other: Self);
    }
    pub trait DiffAlgebra<P>: Sized {
        fn inverse(&self, base: &P) -> Self;
        fn between(base: &P, other: &P) -> Self;
        fn is_empty(&self) -> bool;
    }
    pub trait Mutation<P>: Clone {
        type Diff: MutationDiff<P>;
        fn diff(&self, base: &P) -> Self::Diff;
        fn inverse(&self, base: &P) -> Vec<Self>;
    }
}

//#region xml node model (mirrors 📰xml's real snapshot types)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XmlAttr {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum XmlNode {
    Element { name: String, attrs: Vec<XmlAttr>, children: Vec<XmlNode> },
    Text { text: String },
    CData { text: String },
    Comment { text: String },
    ProcessingInstruction { target: String, data: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XmlDeclaration {
    pub version: String,
    pub encoding: Option<String>,
    pub standalone: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XmlDocument {
    pub root: Option<XmlNode>,
    pub doctype: Option<String>,
    pub declaration: Option<XmlDeclaration>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SvgSnapshot {
    pub schema: String,
    pub doc: XmlDocument,
}
impl Default for SvgSnapshot {
    fn default() -> Self {
        Self { schema: "stdio.svg".into(), doc: XmlDocument::default() }
    }
}

pub type NodePath = Vec<usize>;

pub fn node_at<'a>(doc: &'a XmlDocument, path: &[usize]) -> Result<&'a XmlNode, String> {
    let mut node = doc.root.as_ref().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            XmlNode::Element { children, .. } => {
                node = children.get(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

pub fn element_attr<'a>(node: &'a XmlNode, name: &str) -> Option<&'a str> {
    match node {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str()),
        _ => None,
    }
}
//#endregion

//#region 🔺️ svg diff (verbatim port of the real diff.rs body)
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SvgDiff {
    pub declaration: Option<Option<XmlDeclaration>>,
    pub doctype: Option<Option<String>>,
    pub root: Option<SvgNodeDiff>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SvgNodeDiff {
    Element(SvgElementDiff),
    Text { text: Option<String> },
    Replace { node: Option<XmlNode> },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SvgElementDiff {
    pub name: Option<String>,
    pub attributes: Option<SvgAttributesDiff>,
    pub children: Option<SvgChildrenDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SvgAttributesDiff {
    pub removed: Vec<String>,
    pub modified: Vec<SvgAttrModified>,
    pub added: Vec<SvgAttrAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SvgAttrModified {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SvgAttrAdded {
    pub index: usize,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SvgChildrenDiff {
    pub removed: Vec<usize>,
    pub modified: Vec<SvgChildModified>,
    pub added: Vec<SvgChildAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SvgChildModified {
    pub index: usize,
    pub diff: SvgNodeDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SvgChildAdded {
    pub index: usize,
    pub item: XmlNode,
}

pub fn diff_at_path(path: &[usize], leaf: SvgNodeDiff) -> SvgDiff {
    let mut node_diff = leaf;
    for &index in path.iter().rev() {
        node_diff = SvgNodeDiff::Element(SvgElementDiff {
            name: None,
            attributes: None,
            children: Some(SvgChildrenDiff { removed: Vec::new(), modified: vec![SvgChildModified { index, diff: node_diff }], added: Vec::new() }),
        });
    }
    SvgDiff { declaration: None, doctype: None, root: Some(node_diff) }
}

impl protocol::MutationDiff<SvgSnapshot> for SvgDiff {
    fn apply(&self, base: &SvgSnapshot) -> SvgSnapshot {
        let mut next = base.clone();
        if let Some(declaration) = &self.declaration {
            next.doc.declaration = declaration.clone();
        }
        if let Some(doctype) = &self.doctype {
            next.doc.doctype = doctype.clone();
        }
        if let Some(node_diff) = &self.root {
            next.doc.root = apply_root_diff(next.doc.root.as_ref(), node_diff);
        }
        next
    }
    fn absorb(&mut self, other: Self) {
        if other.declaration.is_some() {
            self.declaration = other.declaration;
        }
        if other.doctype.is_some() {
            self.doctype = other.doctype;
        }
        self.root = match (self.root.take(), other.root) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_node_diff(a, b)),
        };
    }
}

fn apply_root_diff(current: Option<&XmlNode>, diff: &SvgNodeDiff) -> Option<XmlNode> {
    match diff {
        SvgNodeDiff::Replace { node } => node.clone(),
        _ => current.map(|n| apply_node_diff(n, diff)),
    }
}

fn apply_node_diff(node: &XmlNode, diff: &SvgNodeDiff) -> XmlNode {
    match diff {
        SvgNodeDiff::Replace { node: replacement } => replacement.clone().unwrap_or_else(|| node.clone()),
        SvgNodeDiff::Text { text } => match node {
            XmlNode::Text { text: current } => XmlNode::Text { text: text.clone().unwrap_or_else(|| current.clone()) },
            other => other.clone(),
        },
        SvgNodeDiff::Element(element_diff) => match node {
            XmlNode::Element { name, attrs, children } => XmlNode::Element {
                name: element_diff.name.clone().unwrap_or_else(|| name.clone()),
                attrs: match &element_diff.attributes {
                    Some(attrs_diff) => apply_attrs_diff(attrs, attrs_diff),
                    None => attrs.clone(),
                },
                children: match &element_diff.children {
                    Some(children_diff) => apply_children_diff(children, children_diff),
                    None => children.clone(),
                },
            },
            other => other.clone(),
        },
    }
}

fn apply_attrs_diff(attrs: &[XmlAttr], diff: &SvgAttributesDiff) -> Vec<XmlAttr> {
    let mut out: Vec<XmlAttr> = attrs
        .iter()
        .filter(|a| !diff.removed.contains(&a.name))
        .map(|a| match diff.modified.iter().find(|m| m.name == a.name) {
            Some(m) => XmlAttr { name: a.name.clone(), value: m.value.clone() },
            None => a.clone(),
        })
        .collect();
    let mut additions: Vec<&SvgAttrAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, XmlAttr { name: add.name.clone(), value: add.value.clone() });
    }
    out
}

fn apply_children_diff(children: &[XmlNode], diff: &SvgChildrenDiff) -> Vec<XmlNode> {
    let mut slots: Vec<Option<XmlNode>> = children.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(node)) = slots.get(m.index) {
            let patched = apply_node_diff(node, &m.diff);
            slots[m.index] = Some(patched);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < slots.len() {
            slots.remove(idx);
        }
    }
    let mut out: Vec<XmlNode> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&SvgChildAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}

impl protocol::DiffAlgebra<SvgSnapshot> for SvgDiff {
    fn inverse(&self, base: &SvgSnapshot) -> Self {
        SvgDiff {
            declaration: self.declaration.as_ref().map(|_| base.doc.declaration.clone()),
            doctype: self.doctype.as_ref().map(|_| base.doc.doctype.clone()),
            root: self.root.as_ref().map(|d| inverse_node_diff(base.doc.root.as_ref(), d)),
        }
    }
    fn between(base: &SvgSnapshot, other: &SvgSnapshot) -> Self {
        SvgDiff {
            declaration: if base.doc.declaration != other.doc.declaration { Some(other.doc.declaration.clone()) } else { None },
            doctype: if base.doc.doctype != other.doc.doctype { Some(other.doc.doctype.clone()) } else { None },
            root: between_root(base.doc.root.as_ref(), other.doc.root.as_ref()),
        }
    }
    fn is_empty(&self) -> bool {
        self.declaration.is_none() && self.doctype.is_none() && self.root.is_none()
    }
}

fn inverse_node_diff(current: Option<&XmlNode>, diff: &SvgNodeDiff) -> SvgNodeDiff {
    match diff {
        SvgNodeDiff::Replace { .. } => SvgNodeDiff::Replace { node: current.cloned() },
        SvgNodeDiff::Text { .. } => match current {
            Some(XmlNode::Text { text }) => SvgNodeDiff::Text { text: Some(text.clone()) },
            Some(other) => SvgNodeDiff::Replace { node: Some(other.clone()) },
            None => SvgNodeDiff::Replace { node: None },
        },
        SvgNodeDiff::Element(element_diff) => match current {
            Some(XmlNode::Element { name, attrs, children }) => SvgNodeDiff::Element(SvgElementDiff {
                name: element_diff.name.as_ref().map(|_| name.clone()),
                attributes: element_diff.attributes.as_ref().map(|ad| inverse_attrs_diff(attrs, ad)),
                children: element_diff.children.as_ref().map(|cd| inverse_children_diff(children, cd)),
            }),
            Some(other) => SvgNodeDiff::Replace { node: Some(other.clone()) },
            None => SvgNodeDiff::Replace { node: None },
        },
    }
}

fn inverse_attrs_diff(base_attrs: &[XmlAttr], diff: &SvgAttributesDiff) -> SvgAttributesDiff {
    let removed: Vec<String> = diff.added.iter().map(|a| a.name.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_attrs.iter().find(|a| a.name == m.name) {
            modified.push(SvgAttrModified { name: original.name.clone(), value: original.value.clone() });
        }
    }
    let mut added = Vec::new();
    for name in &diff.removed {
        if let Some(idx) = base_attrs.iter().position(|a| &a.name == name) {
            let original = &base_attrs[idx];
            added.push(SvgAttrAdded { index: idx, name: original.name.clone(), value: original.value.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    SvgAttributesDiff { removed, modified, added }
}

fn inverse_children_diff(base_children: &[XmlNode], diff: &SvgChildrenDiff) -> SvgChildrenDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_children.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(SvgChildModified { index: next_index, diff: inverse_node_diff(Some(original), &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_children.get(idx) {
            added.push(SvgChildAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    SvgChildrenDiff { removed, modified, added }
}

fn between_root(base: Option<&XmlNode>, other: Option<&XmlNode>) -> Option<SvgNodeDiff> {
    match (base, other) {
        (None, None) => None,
        (None, Some(n)) => Some(SvgNodeDiff::Replace { node: Some(n.clone()) }),
        (Some(_), None) => Some(SvgNodeDiff::Replace { node: None }),
        (Some(b), Some(o)) => between_node(b, o),
    }
}

fn between_node(base: &XmlNode, other: &XmlNode) -> Option<SvgNodeDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (XmlNode::Text { .. }, XmlNode::Text { text: ot }) => Some(SvgNodeDiff::Text { text: Some(ot.clone()) }),
        (XmlNode::Element { name: bn, attrs: ba, children: bc }, XmlNode::Element { name: on, attrs: oa, children: oc }) => {
            let name = if bn != on { Some(on.clone()) } else { None };
            let attributes = between_attrs(ba, oa);
            let children = between_children(bc, oc);
            if name.is_none() && attributes.is_none() && children.is_none() {
                None
            } else {
                Some(SvgNodeDiff::Element(SvgElementDiff { name, attributes, children }))
            }
        }
        _ => Some(SvgNodeDiff::Replace { node: Some(other.clone()) }),
    }
}

fn between_attrs(base: &[XmlAttr], other: &[XmlAttr]) -> Option<SvgAttributesDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        match other.iter().find(|o| o.name == b.name) {
            Some(o) if o.value != b.value => modified.push(SvgAttrModified { name: b.name.clone(), value: o.value.clone() }),
            Some(_) => {}
            None => removed.push(b.name.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, o) in other.iter().enumerate() {
        if !base.iter().any(|b| b.name == o.name) {
            added.push(SvgAttrAdded { index: i, name: o.name.clone(), value: o.value.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(SvgAttributesDiff { removed, modified, added }) }
}

fn between_children(base: &[XmlNode], other: &[XmlNode]) -> Option<SvgChildrenDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_node(&base[i], &other[i]) {
                modified.push(SvgChildModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<SvgChildAdded> = (min_len..other.len()).map(|i| SvgChildAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(SvgChildrenDiff { removed, modified, added }) }
}

fn transform_index(idx: usize, removed: &[usize], added: &[SvgChildAdded]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

enum ChildOrigin {
    Base(usize),
    Added(usize),
}

fn simulate_mid_origins(base_len: usize, removed: &[usize], added: &[SvgChildAdded]) -> Vec<ChildOrigin> {
    let mut mid: Vec<ChildOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ChildOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ChildOrigin::Added(k));
    }
    mid
}

fn absorb_node_diff(a: SvgNodeDiff, b: SvgNodeDiff) -> SvgNodeDiff {
    match (a, b) {
        (_, SvgNodeDiff::Replace { node: Some(n) }) => SvgNodeDiff::Replace { node: Some(n) },
        (SvgNodeDiff::Replace { node: Some(n) }, b) => SvgNodeDiff::Replace { node: Some(apply_node_diff(&n, &b)) },
        (_, SvgNodeDiff::Replace { node: None }) => SvgNodeDiff::Replace { node: None },
        (SvgNodeDiff::Replace { node: None }, _) => SvgNodeDiff::Replace { node: None },
        (SvgNodeDiff::Text { text: ta }, SvgNodeDiff::Text { text: tb }) => SvgNodeDiff::Text { text: tb.or(ta) },
        (SvgNodeDiff::Element(ea), SvgNodeDiff::Element(eb)) => SvgNodeDiff::Element(absorb_element_diff(ea, eb)),
        (_, b) => b,
    }
}

fn absorb_element_diff(mut a: SvgElementDiff, b: SvgElementDiff) -> SvgElementDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    a.attributes = match (a.attributes.take(), b.attributes) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(absorb_attrs_diff(ad, bd)),
    };
    a.children = match (a.children.take(), b.children) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(absorb_children_diff(ad, bd)),
    };
    a
}

fn absorb_attrs_diff(mut a: SvgAttributesDiff, b: SvgAttributesDiff) -> SvgAttributesDiff {
    let a_added_names: std::collections::HashSet<String> = a.added.iter().map(|x| x.name.clone()).collect();
    let mut removed = a.removed.clone();
    let mut annihilated: Vec<String> = Vec::new();
    for name in &b.removed {
        if a_added_names.contains(name) {
            annihilated.push(name.clone());
        } else if !removed.contains(name) {
            removed.push(name.clone());
        }
    }
    a.added.retain(|x| !annihilated.contains(&x.name));
    let mut modified: Vec<SvgAttrModified> = a.modified.into_iter().filter(|m| !removed.contains(&m.name)).collect();
    for bm in &b.modified {
        if let Some(added) = a.added.iter_mut().find(|x| x.name == bm.name) {
            added.value = bm.value.clone();
            continue;
        }
        if removed.contains(&bm.name) {
            continue;
        }
        match modified.iter_mut().find(|m| m.name == bm.name) {
            Some(existing) => existing.value = bm.value.clone(),
            None => modified.push(bm.clone()),
        }
    }
    let mut added = a.added;
    for ba in &b.added {
        match added.iter_mut().find(|x| x.name == ba.name) {
            Some(existing) => {
                existing.value = ba.value.clone();
                existing.index = ba.index;
            }
            None => added.push(ba.clone()),
        }
    }
    added.sort_by_key(|x| x.index);
    SvgAttributesDiff { removed, modified, added }
}

fn absorb_children_diff(d1: SvgChildrenDiff, d2: SvgChildrenDiff) -> SvgChildrenDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added.clone();
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ChildOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ChildOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ChildOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_node_diff(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(SvgChildModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ChildOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_node_diff(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(SvgChildAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    SvgChildrenDiff { removed, modified, added }
}

pub fn diff_set_snapshot(base: &SvgSnapshot, next: &SvgSnapshot) -> SvgDiff {
    <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(base, next)
}
//#endregion

//#region 🧬️ svg mutations (verbatim port of the real mutations.rs body, minus OpText/OpBinary)
#[derive(Clone, Debug, Default, PartialEq)]
pub enum SvgMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: SvgSnapshot },
    SetDeclaration { declaration: Option<XmlDeclaration> },
    SetDoctype { doctype: Option<String> },
    InsertElement { parent: NodePath, index: usize, node: XmlNode },
    RemoveElement { parent: NodePath, index: usize },
    SetElementName { path: NodePath, name: String },
    SetAttribute { path: NodePath, name: String, value: Option<String> },
    SetText { path: NodePath, text: String },
}

pub fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) -> SvgDiff {
    let diff = <SvgMutation as protocol::Mutation<SvgSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, snapshot);
    diff
}

fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<String>) -> SvgDiff {
    let target = node_at(&base.doc, path).ok();
    let existing = target.and_then(|n| match n {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|a| a.name == name),
        _ => None,
    });
    let attrs_diff = match (existing, value) {
        (Some(_), Some(v)) => SvgAttributesDiff { removed: Vec::new(), modified: vec![SvgAttrModified { name: name.to_string(), value: v }], added: Vec::new() },
        (Some(_), None) => SvgAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (None, Some(v)) => {
            let next_index = match target {
                Some(XmlNode::Element { attrs, .. }) => attrs.len(),
                _ => 0,
            };
            SvgAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgAttrAdded { index: next_index, name: name.to_string(), value: v }] }
        }
        (None, None) => SvgAttributesDiff::default(),
    };
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(attrs_diff), children: None }))
}

fn prior_attribute(base: &SvgSnapshot, path: &[usize], name: &str) -> Option<String> {
    node_at(&base.doc, path).ok().and_then(|n| element_attr(n, name)).map(|s| s.to_string())
}

impl protocol::Mutation<SvgSnapshot> for SvgMutation {
    type Diff = SvgDiff;

    fn diff(&self, base: &SvgSnapshot) -> Self::Diff {
        match self {
            SvgMutation::NoMutation => SvgDiff::default(),
            SvgMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SvgMutation::SetDeclaration { declaration } => SvgDiff { declaration: Some(declaration.clone()), doctype: None, root: None },
            SvgMutation::SetDoctype { doctype } => SvgDiff { declaration: None, doctype: Some(doctype.clone()), root: None },
            SvgMutation::InsertElement { parent, index, node } => diff_at_path(
                parent,
                SvgNodeDiff::Element(SvgElementDiff {
                    name: None,
                    attributes: None,
                    children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index: *index, item: node.clone() }] }),
                }),
            ),
            SvgMutation::RemoveElement { parent, index } => diff_at_path(
                parent,
                SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) }),
            ),
            SvgMutation::SetElementName { path, name } => diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: Some(name.clone()), attributes: None, children: None })),
            SvgMutation::SetAttribute { path, name, value } => attribute_diff_at_path(base, path, name, value.clone()),
            SvgMutation::SetText { path, text } => diff_at_path(path, SvgNodeDiff::Text { text: Some(text.clone()) }),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<Self> {
        match self {
            SvgMutation::NoMutation => vec![SvgMutation::NoMutation],
            SvgMutation::SetSnapshot { .. } => vec![SvgMutation::SetSnapshot { snapshot: base.clone() }],
            SvgMutation::SetDeclaration { .. } => vec![SvgMutation::SetDeclaration { declaration: base.doc.declaration.clone() }],
            SvgMutation::SetDoctype { .. } => vec![SvgMutation::SetDoctype { doctype: base.doc.doctype.clone() }],
            SvgMutation::InsertElement { parent, index, .. } => vec![SvgMutation::RemoveElement { parent: parent.clone(), index: *index }],
            SvgMutation::RemoveElement { parent, index } => match node_at(&base.doc, parent) {
                Ok(XmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => vec![SvgMutation::InsertElement { parent: parent.clone(), index: *index, node: node.clone() }],
                    None => vec![SvgMutation::NoMutation],
                },
                _ => vec![SvgMutation::NoMutation],
            },
            SvgMutation::SetElementName { path, .. } => {
                let prior = match node_at(&base.doc, path) {
                    Ok(XmlNode::Element { name, .. }) => name.clone(),
                    _ => return vec![SvgMutation::NoMutation],
                };
                vec![SvgMutation::SetElementName { path: path.clone(), name: prior }]
            }
            SvgMutation::SetAttribute { path, name, .. } => vec![SvgMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) }],
            SvgMutation::SetText { path, .. } => {
                let old = match node_at(&base.doc, path) {
                    Ok(XmlNode::Text { text }) => text.clone(),
                    _ => String::new(),
                };
                vec![SvgMutation::SetText { path: path.clone(), text: old }]
            }
        }
    }
}
//#endregion

//#region tests (mirrors the real crate's mutations.rs test region, minus DSL-parse-based fixtures)
fn fixture() -> SvgSnapshot {
    SvgSnapshot {
        schema: "stdio.svg".into(),
        doc: XmlDocument {
            declaration: None,
            doctype: None,
            root: Some(XmlNode::Element {
                name: "svg".into(),
                attrs: vec![XmlAttr { name: "xmlns".into(), value: "http://www.w3.org/2000/svg".into() }, XmlAttr { name: "viewBox".into(), value: "0 0 10 10".into() }],
                children: vec![XmlNode::Element {
                    name: "rect".into(),
                    attrs: vec![
                        XmlAttr { name: "x".into(), value: "0".into() },
                        XmlAttr { name: "y".into(), value: "0".into() },
                        XmlAttr { name: "width".into(), value: "5".into() },
                        XmlAttr { name: "height".into(), value: "5".into() },
                    ],
                    children: vec![],
                }],
            }),
        },
    }
}

fn sample_mutations() -> Vec<SvgMutation> {
    vec![
        SvgMutation::NoMutation,
        SvgMutation::SetDeclaration { declaration: Some(XmlDeclaration { version: "1.1".into(), encoding: Some("UTF-8".into()), standalone: Some(false) }) },
        SvgMutation::SetDeclaration { declaration: None },
        SvgMutation::SetDoctype { doctype: Some("<!DOCTYPE foo>".into()) },
        SvgMutation::InsertElement { parent: vec![], index: 1, node: XmlNode::Element { name: "circle".into(), attrs: Vec::new(), children: Vec::new() } },
        SvgMutation::RemoveElement { parent: vec![], index: 0 },
        SvgMutation::SetElementName { path: vec![0], name: "circle".into() },
        SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some("99".into()) },
        SvgMutation::SetAttribute { path: vec![0], name: "height".into(), value: None },
        SvgMutation::SetText { path: vec![], text: "hi".into() },
    ]
}

fn mutation_diff_law() {
    for mutation in sample_mutations() {
        let base = fixture();
        let diff_direct = protocol::Mutation::diff(&mutation, &base);
        let applied_via_diff = protocol::MutationDiff::apply(&diff_direct, &base);

        let mut via_apply = base.clone();
        let diff_from_apply = apply_svg_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
    println!("mutation_diff_law: OK");
}

fn inverse_law() {
    for mutation in sample_mutations() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_svg_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in protocol::Mutation::inverse(&mutation, &base) {
            apply_svg_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

        let diff = protocol::Mutation::diff(&mutation, &base);
        let next = protocol::MutationDiff::apply(&diff, &base);
        let inverse_diff = protocol::DiffAlgebra::inverse(&diff, &base);
        let restored = protocol::MutationDiff::apply(&inverse_diff, &next);
        assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
    }
    println!("inverse_law: OK");
}

fn inverse_diff_level_restores_middle_attribute_position() {
    let base = fixture();
    let mutation = SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: None };
    let diff = protocol::Mutation::diff(&mutation, &base);
    let next = protocol::MutationDiff::apply(&diff, &base);
    assert_eq!(element_attr(node_at(&next.doc, &[0]).unwrap(), "width"), None);

    let inverse_diff = protocol::DiffAlgebra::inverse(&diff, &base);
    let restored = protocol::MutationDiff::apply(&inverse_diff, &next);
    assert_eq!(restored, base, "diff-level inverse must restore the exact original attribute order");
    println!("inverse_diff_level_restores_middle_attribute_position: OK");
}

fn two_child_root(a_name: &str, b_name: &str) -> SvgSnapshot {
    SvgSnapshot {
        schema: "stdio.svg".into(),
        doc: XmlDocument {
            declaration: None,
            doctype: None,
            root: Some(XmlNode::Element {
                name: "svg".into(),
                attrs: Vec::new(),
                children: vec![
                    XmlNode::Element { name: a_name.into(), attrs: Vec::new(), children: Vec::new() },
                    XmlNode::Element { name: b_name.into(), attrs: Vec::new(), children: Vec::new() },
                ],
            }),
        },
    }
}

fn assert_absorb_matches_sequential(base: &SvgSnapshot, d1: &SvgDiff, d2: &SvgDiff) -> SvgDiff {
    let sequential = protocol::MutationDiff::apply(d2, &protocol::MutationDiff::apply(d1, base));
    let mut absorbed = d1.clone();
    protocol::MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(protocol::MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

fn root_children_diff(diff: &SvgDiff) -> &SvgChildrenDiff {
    match diff.root.as_ref().expect("root diff present") {
        SvgNodeDiff::Element(e) => e.children.as_ref().expect("children diff present"),
        other => panic!("expected element diff, got {other:?}"),
    }
}

fn absorb_law() {
    {
        let base = two_child_root("a", "b");
        let d1 = protocol::Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = protocol::Mutation::diff(&SvgMutation::RemoveElement { parent: vec![], index: 0 }, &mid);
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let triple = root_children_diff(&absorbed);
        assert_eq!(triple.removed, vec![0]);
        assert_eq!(triple.added.len(), 1);
        assert_eq!(triple.added[0].index, 1);
        let XmlNode::Element { name, .. } = &triple.added[0].item else { panic!("expected element") };
        assert_eq!(name, "f");
    }
    {
        let base = two_child_root("a", "b");
        let d1 = protocol::Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = protocol::Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid);
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let triple = root_children_diff(&absorbed);
        assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
    }
    {
        let base = two_child_root("a", "b");
        let d1 = protocol::Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 1, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = protocol::Mutation::diff(&SvgMutation::SetAttribute { path: vec![1], name: "k".into(), value: Some("v".into()) }, &mid);
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let triple = root_children_diff(&absorbed);
        assert!(triple.modified.is_empty());
        assert_eq!(triple.added.len(), 1);
        let XmlNode::Element { attrs, .. } = &triple.added[0].item else { panic!("expected element") };
        assert!(attrs.iter().any(|a| a.name == "k" && a.value == "v"));
    }
    {
        let base = two_child_root("a", "b");
        let d1 = protocol::Mutation::diff(&SvgMutation::SetAttribute { path: vec![1], name: "k".into(), value: Some("v".into()) }, &base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = protocol::Mutation::diff(&SvgMutation::RemoveElement { parent: vec![], index: 1 }, &mid);
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let triple = root_children_diff(&absorbed);
        assert!(triple.modified.is_empty());
        assert_eq!(triple.removed, vec![1]);
    }
    {
        let base = two_child_root("a", "b");
        let d1 = protocol::Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
        let mid1 = protocol::MutationDiff::apply(&d1, &base);
        let d2 = protocol::Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid1);
        let mid2 = protocol::MutationDiff::apply(&d2, &mid1);
        let d3 = protocol::Mutation::diff(&SvgMutation::RemoveElement { parent: vec![], index: 0 }, &mid2);
        let sequential = protocol::MutationDiff::apply(&d3, &mid2);

        let mut left = d1.clone();
        protocol::MutationDiff::absorb(&mut left, d2.clone());
        protocol::MutationDiff::absorb(&mut left, d3.clone());

        let mut d2_then_d3 = d2.clone();
        protocol::MutationDiff::absorb(&mut d2_then_d3, d3.clone());
        let mut right = d1.clone();
        protocol::MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(protocol::MutationDiff::apply(&left, &base), sequential);
        assert_eq!(protocol::MutationDiff::apply(&right, &base), sequential);
    }
    println!("absorb_law: OK");
}

fn sweep_a() -> SvgSnapshot {
    SvgSnapshot {
        schema: "stdio.svg".into(),
        doc: XmlDocument {
            declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
            doctype: Some("<!DOCTYPE svg>".into()),
            root: Some(XmlNode::Element {
                name: "svg".into(),
                attrs: vec![
                    XmlAttr { name: "keep".into(), value: "k".into() },
                    XmlAttr { name: "toRemove".into(), value: "r".into() },
                    XmlAttr { name: "toModify".into(), value: "old".into() },
                ],
                children: vec![
                    XmlNode::Element {
                        name: "g".into(),
                        attrs: vec![XmlAttr { name: "x".into(), value: "1".into() }],
                        children: vec![XmlNode::Element { name: "rect".into(), attrs: Vec::new(), children: Vec::new() }],
                    },
                    XmlNode::Text { text: "stay".into() },
                    XmlNode::Element { name: "toDrop".into(), attrs: Vec::new(), children: Vec::new() },
                ],
            }),
        },
    }
}

fn sweep_b() -> SvgSnapshot {
    SvgSnapshot {
        schema: "stdio.svg".into(),
        doc: XmlDocument {
            declaration: None,
            doctype: None,
            root: Some(XmlNode::Element {
                name: "svgRenamed".into(),
                attrs: vec![
                    XmlAttr { name: "keep".into(), value: "k".into() },
                    XmlAttr { name: "toModify".into(), value: "new".into() },
                    XmlAttr { name: "added".into(), value: "a".into() },
                ],
                children: vec![
                    XmlNode::Element {
                        name: "gModified".into(),
                        attrs: vec![XmlAttr { name: "x".into(), value: "2".into() }, XmlAttr { name: "y".into(), value: "3".into() }],
                        children: vec![
                            XmlNode::Element { name: "rect".into(), attrs: Vec::new(), children: Vec::new() },
                            XmlNode::Element { name: "circle".into(), attrs: Vec::new(), children: Vec::new() },
                        ],
                    },
                    XmlNode::Text { text: "stay".into() },
                ],
            }),
        },
    }
}

fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(protocol::MutationDiff::apply(&<SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(&a, &b), &a), b);
    assert_eq!(protocol::MutationDiff::apply(&<SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(&b, &a), &b), a);
    let sample = fixture();
    assert_eq!(protocol::MutationDiff::apply(&<SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(&sample, &sample), &sample), sample);
    println!("between_roundtrip_law: OK");
}

fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(&a, &b);
    assert_eq!(protocol::MutationDiff::apply(&diff_ab, &a), b);
    let diff_ba = <SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(&b, &a);
    assert_eq!(protocol::MutationDiff::apply(&diff_ba, &b), a);
    assert!(<SvgDiff as protocol::DiffAlgebra<SvgSnapshot>>::between(&a, &a).is_empty());

    assert_eq!(diff_ab.declaration, Some(None));
    assert_eq!(diff_ab.doctype, Some(None));
    assert!(diff_ab.root.is_some());

    let SvgNodeDiff::Element(root_diff) = diff_ab.root.as_ref().unwrap() else { panic!("expected element diff") };
    assert!(root_diff.name.is_some());
    let attrs_diff = root_diff.attributes.as_ref().expect("attrs diff present");
    assert!(!attrs_diff.removed.is_empty());
    assert!(!attrs_diff.modified.is_empty());
    assert!(!attrs_diff.added.is_empty());

    let children_diff = root_diff.children.as_ref().expect("children diff present");
    assert!(!children_diff.removed.is_empty());
    assert_eq!(children_diff.modified.len(), 1);
    let modified_entry = &children_diff.modified[0];
    let SvgNodeDiff::Element(modified_element) = &modified_entry.diff else { panic!("expected element diff") };
    assert!(modified_element.name.is_some());
    assert!(modified_element.attributes.is_some());
    let nested_children = modified_element.children.as_ref().expect("nested children diff present");
    assert!(!nested_children.added.is_empty());

    println!("field_sweep: OK");
}
//#endregion

fn main() {
    mutation_diff_law();
    inverse_law();
    inverse_diff_level_restores_middle_attribute_position();
    absorb_law();
    between_roundtrip_law();
    field_sweep();
    println!("ALL SCRATCH LAWS PASSED");
}
