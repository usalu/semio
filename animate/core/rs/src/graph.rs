//! 🕸️ Graph and directed graph layouts as Sobject groups.

use crate::color::Color;
use crate::geometry::{arrow, circle, line};
use crate::sobject::{Group, Sobject};
use crate::text::Text;
use mathematical_geometry::Point;
use std::collections::HashMap;

/// 🔵 Undirected graph with circular layout.
pub struct Graph {
    pub group: Group,
    pub nodes: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

impl Graph {
    pub fn new(nodes: Vec<u32>, edges: Vec<(u32, u32)>, radius: f64, center: Point, color: Color) -> Self {
        let positions = circular_layout(&nodes, radius, center);
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for &(a, b) in &edges {
            if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                children.push(Box::new(line(pa, pb, color.with_alpha(0.6), 2.0)));
            }
        }
        for &n in &nodes {
            if let Some(&p) = positions.get(&n) {
                children.push(Box::new(circle(p, 0.2, color, None, 0.0)));
            }
        }
        Self {
            group: Group::new(children),
            nodes,
            edges,
        }
    }

    pub fn with_edge_labels(mut self, labels: &HashMap<(u32, u32), String>, positions: &HashMap<u32, Point>, color: Color) -> Self {
        for (&(a, b), label) in labels {
            if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                let mid = Point::new((pa.x() + pb.x()) / 2.0, (pa.y() + pb.y()) / 2.0);
                let mut t = Text::new(label, color);
                t.inner.move_to(mid);
                self.group.add_child(Box::new(t.inner));
            }
        }
        self
    }
}

/// ➡️ Directed graph with force-directed layout seed.
pub struct DiGraph {
    pub group: Group,
    pub nodes: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

impl DiGraph {
    pub fn new(nodes: Vec<u32>, edges: Vec<(u32, u32)>, radius: f64, center: Point, color: Color) -> Self {
        let positions = force_layout_seed(&nodes, &edges, radius, center);
        let node_r = 0.18;
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for &(a, b) in &edges {
            if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                let dir = pb - pa;
                let len = dir.hypot().max(1e-9);
                let u = dir / len;
                let start = pa + u * node_r;
                let end = pb - u * node_r;
                children.push(Box::new(arrow(start, end, color.with_alpha(0.7), 2.0, 0.15)));
            }
        }
        for &n in &nodes {
            if let Some(&p) = positions.get(&n) {
                children.push(Box::new(circle(p, node_r, color, Some(Color::WHITE), 1.0)));
            }
        }
        Self {
            group: Group::new(children),
            nodes,
            edges,
        }
    }

    pub fn with_edge_labels(mut self, labels: &HashMap<(u32, u32), String>, positions: &HashMap<u32, Point>, color: Color) -> Self {
        for (&(a, b), label) in labels {
            if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                let mid = Point::new((pa.x() + pb.x()) / 2.0, (pa.y() + pb.y()) / 2.0);
                let mut t = Text::new(label, color);
                t.inner.move_to(mid);
                self.group.add_child(Box::new(t.inner));
            }
        }
        self
    }
}

fn circular_layout(nodes: &[u32], radius: f64, center: Point) -> HashMap<u32, Point> {
    let mut out = HashMap::new();
    let n = nodes.len().max(1);
    for (i, &id) in nodes.iter().enumerate() {
        let t = i as f64 / n as f64 * std::f64::consts::TAU;
        out.insert(id, Point::new(center.x() + radius * t.cos(), center.y() + radius * t.sin()));
    }
    out
}

fn force_layout_seed(nodes: &[u32], edges: &[(u32, u32)], radius: f64, center: Point) -> HashMap<u32, Point> {
    let mut pos = circular_layout(nodes, radius, center);
    for _ in 0..24 {
        let mut forces: HashMap<u32, (f64, f64)> = nodes.iter().map(|&id| (id, (0.0, 0.0))).collect();
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                let pa = pos[&a];
                let pb = pos[&b];
                let dx = pb.x() - pa.x();
                let dy = pb.y() - pa.y();
                let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                let rep = 0.05 / dist;
                forces.get_mut(&a).unwrap().0 -= dx * rep;
                forces.get_mut(&a).unwrap().1 -= dy * rep;
                forces.get_mut(&b).unwrap().0 += dx * rep;
                forces.get_mut(&b).unwrap().1 += dy * rep;
            }
        }
        for &(a, b) in edges {
            let pa = pos[&a];
            let pb = pos[&b];
            let dx = pb.x() - pa.x();
            let dy = pb.y() - pa.y();
            let dist = (dx * dx + dy * dy).sqrt().max(0.01);
            let att = dist * 0.02;
            forces.get_mut(&a).unwrap().0 += dx / dist * att;
            forces.get_mut(&a).unwrap().1 += dy / dist * att;
            forces.get_mut(&b).unwrap().0 -= dx / dist * att;
            forces.get_mut(&b).unwrap().1 -= dy / dist * att;
        }
        for &id in nodes {
            let (fx, fy) = forces[&id];
            let p = pos.get_mut(&id).unwrap();
            *p = Point::new(p.x() + fx, p.y() + fy);
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_has_node_and_edge_children() {
        let g = Graph::new(vec![1, 2, 3], vec![(1, 2), (2, 3)], 2.0, Point::ZERO, Color::BLUE);
        assert_eq!(g.nodes.len(), 3);
        assert!(!g.group.children.is_empty());
    }

    #[test]
    fn digraph_uses_arrows_and_labels() {
        let dg = DiGraph::new(vec![1, 2], vec![(1, 2)], 2.0, Point::ZERO, Color::WHITE);
        assert_eq!(dg.edges.len(), 1);
        let mut positions = HashMap::new();
        positions.insert(1, Point::new(-1.0, 0.0));
        positions.insert(2, Point::new(1.0, 0.0));
        let mut labels = HashMap::new();
        labels.insert((1, 2), "edge".into());
        let labeled = dg.with_edge_labels(&labels, &positions, Color::WHITE);
        assert!(labeled.group.children.len() > 2);
    }
}
