// 🧪️ [DEBUG] Standalone runtime check of the IoRouter route-resolution algorithm shipped in
// 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs (region 🔖️IoRouter /
// 🔖️IoRouterW1d). The real crate's `cargo nextest run -p semio-framework-plugin-host --lib` is
// blocked by a PRE-EXISTING, out-of-boundary compile error (proven via git show at the ticket's
// start commit — see 🔧️patches/w1d-opening-config-mutations-missing-default-app-import.txt), so
// this file copies the exact algorithm (resolve_io_route/walk_io_routes/io_route_rank/
// rank_to_io_fidelity/route_reenters_calling_plugin/io_entries_conflict, byte-for-byte from the
// shipped file) plus minimal standalone twins of ArtifactDialect/IoFidelity, to get REAL execution
// evidence for the algorithm ahead of the patch landing. Run: rustc w1d_io_router_algorithm_check.rs -o /tmp/w1d_check && /tmp/w1d_check
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ArtifactDialect { artifact_kind: String, standard: String, subset: String }
impl ArtifactDialect {
    fn to_coordinate(&self) -> String { format!("{}@{}/{}", self.artifact_kind, self.standard, self.subset) }
    fn new(kind: &str, standard: &str, subset: &str) -> Self { Self { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IoFidelity { Exact, Canonical, Semantic, Lossy }
impl IoFidelity {
    fn rank(self) -> u8 { match self { Self::Exact => 3, Self::Canonical => 2, Self::Semantic => 1, Self::Lossy => 0 } }
}

#[derive(Clone, Debug, PartialEq)]
struct IoEntryDescriptor { from: ArtifactDialect, into: ArtifactDialect, fidelity: IoFidelity, sniffs: bool }

#[derive(Clone, Debug, PartialEq)]
struct IoRoute { hops: Vec<IoEntryDescriptor>, fidelity: IoFidelity }

type IoEntryKey = (ArtifactDialect, ArtifactDialect);

#[derive(Clone, Debug, PartialEq)]
struct IoEntryRoute { owner: String, fidelity: IoFidelity, sniffs: bool }

#[derive(Debug)]
struct RouteError(String);

// ---- verbatim copy of the shipped algorithm (component.rs region 🔖️IoRouter) ----

fn rank_to_io_fidelity(rank: u8) -> IoFidelity {
    match rank { 3 => IoFidelity::Exact, 2 => IoFidelity::Canonical, 1 => IoFidelity::Semantic, _ => IoFidelity::Lossy }
}

fn io_route_rank(hops: &[IoEntryDescriptor]) -> (std::cmp::Reverse<u8>, usize, String) {
    let min_fidelity = hops.iter().map(|hop| hop.fidelity.rank()).min().unwrap_or(0);
    let joined = hops.iter().map(|hop| hop.into.to_coordinate()).collect::<Vec<_>>().join(",");
    (std::cmp::Reverse(min_fidelity), hops.len(), joined)
}

fn walk_io_routes(
    graph: &BTreeMap<IoEntryKey, IoEntryRoute>,
    current: &ArtifactDialect,
    into: &ArtifactDialect,
    remaining_hops: u8,
    path: &mut Vec<IoEntryDescriptor>,
    visited: &mut BTreeSet<ArtifactDialect>,
    candidates: &mut Vec<Vec<IoEntryDescriptor>>,
) {
    if remaining_hops == 0 { return; }
    for ((from, hop_into), route) in graph.iter() {
        if from != current || visited.contains(hop_into) { continue; }
        let descriptor = IoEntryDescriptor { from: from.clone(), into: hop_into.clone(), fidelity: route.fidelity, sniffs: route.sniffs };
        path.push(descriptor);
        if hop_into == into {
            candidates.push(path.clone());
        } else {
            visited.insert(hop_into.clone());
            walk_io_routes(graph, hop_into, into, remaining_hops - 1, path, visited, candidates);
            visited.remove(hop_into);
        }
        path.pop();
    }
}

fn resolve_io_route(graph: &BTreeMap<IoEntryKey, IoEntryRoute>, from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8) -> Result<IoRoute, RouteError> {
    let max_hops = max_hops.min(3);
    if max_hops == 0 { return Err(RouteError(format!("io_routes {} -> {}: max_hops clamped to 0", from.to_coordinate(), into.to_coordinate()))); }
    let mut candidates: Vec<Vec<IoEntryDescriptor>> = Vec::new();
    let mut path: Vec<IoEntryDescriptor> = Vec::new();
    let mut visited: BTreeSet<ArtifactDialect> = BTreeSet::new();
    visited.insert(from.clone());
    walk_io_routes(graph, from, into, max_hops, &mut path, &mut visited, &mut candidates);
    if candidates.is_empty() { return Err(RouteError(format!("no io route from {} to {} within {max_hops} hops", from.to_coordinate(), into.to_coordinate()))); }
    candidates.sort_by(|a, b| io_route_rank(a).cmp(&io_route_rank(b)));
    let best = candidates.into_iter().next().expect("candidates checked non-empty above");
    let fidelity = rank_to_io_fidelity(best.iter().map(|hop| hop.fidelity.rank()).min().expect("a route has at least one hop"));
    Ok(IoRoute { hops: best, fidelity })
}

fn route_reenters_calling_plugin<'route>(graph: &BTreeMap<IoEntryKey, IoEntryRoute>, route: &'route IoRoute, calling_plugin_id: &str) -> Option<(&'route ArtifactDialect, &'route ArtifactDialect)> {
    route.hops.iter().find_map(|hop| {
        let owner = &graph.get(&(hop.from.clone(), hop.into.clone()))?.owner;
        (owner == calling_plugin_id).then_some((&hop.from, &hop.into))
    })
}

fn io_entries_conflict(existing: &BTreeMap<IoEntryKey, IoEntryRoute>, plugin_id: &str, incoming: &[IoEntryDescriptor]) -> Option<String> {
    for descriptor in incoming {
        let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
        if let Some(current) = existing.get(&key) {
            if current.owner != plugin_id {
                return Some(format!("io entry route conflict for {:?} -> {:?}: {} already owns it; {} cannot replace it", key.0, key.1, current.owner, plugin_id));
            }
        }
    }
    None
}

// ---- fixture (identical to component.rs's io_router_w1d_fixture_entries) ----

fn fixture_entries() -> Vec<(&'static str, IoEntryDescriptor)> {
    let binary_raw = ArtifactDialect::new("s.stdio.binary", "raw", "*");
    let gif_87a = ArtifactDialect::new("s.stdio.gif", "87a", "*");
    let gif_89a = ArtifactDialect::new("s.stdio.gif", "89a", "*");
    vec![
        ("stdio", IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: IoFidelity::Exact, sniffs: true }),
        ("gif", IoEntryDescriptor { from: gif_87a, into: gif_89a.clone(), fidelity: IoFidelity::Canonical, sniffs: false }),
        ("gif", IoEntryDescriptor { from: binary_raw, into: gif_89a, fidelity: IoFidelity::Lossy, sniffs: true }),
    ]
}

fn build_graph(rows: &[(&'static str, IoEntryDescriptor)]) -> BTreeMap<IoEntryKey, IoEntryRoute> {
    let mut graph = BTreeMap::new();
    for (owner, descriptor) in rows {
        let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
        graph.entry(key).or_insert(IoEntryRoute { owner: (*owner).to_string(), fidelity: descriptor.fidelity, sniffs: descriptor.sniffs });
    }
    graph
}

fn main() {
    let mut failures = 0u32;
    macro_rules! check {
        ($label:expr, $cond:expr) => {
            if $cond { println!("[ok] {}", $label); } else { failures += 1; println!("[FAIL] {}", $label); }
        };
    }

    let binary_raw = ArtifactDialect::new("s.stdio.binary", "raw", "*");
    let gif_87a = ArtifactDialect::new("s.stdio.gif", "87a", "*");
    let gif_89a = ArtifactDialect::new("s.stdio.gif", "89a", "*");

    // determinism across load order
    let forward = fixture_entries();
    let mut reversed = forward.clone();
    reversed.reverse();
    let graph_forward = build_graph(&forward);
    let graph_reversed = build_graph(&reversed);
    check!("merged graph identical regardless of registration order", graph_forward == graph_reversed);
    let route_forward = resolve_io_route(&graph_forward, &binary_raw, &gif_89a, 3).expect("forward route resolves");
    let route_reversed = resolve_io_route(&graph_reversed, &binary_raw, &gif_89a, 3).expect("reversed route resolves");
    check!("resolved route identical regardless of registration order", route_forward == route_reversed);
    check!("winning route is the 2-hop path, not the 1-hop lossy shortcut", route_forward.hops.len() == 2);

    // prefers higher minimum fidelity over fewer hops
    let graph = build_graph(&fixture_entries());
    let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).expect("route resolves");
    check!("route fidelity is Canonical (min of Exact,Canonical)", route.fidelity == IoFidelity::Canonical);
    check!("route has 2 hops", route.hops.len() == 2);
    check!("first hop starts at binary carrier", route.hops[0].from == binary_raw);
    check!("last hop ends at gif89a", route.hops[1].into == gif_89a);

    // max_hops bound
    let route_1hop = resolve_io_route(&graph, &binary_raw, &gif_89a, 1).expect("1-hop route resolves");
    check!("bounded to 1 hop picks the direct lossy shortcut", route_1hop.hops.len() == 1 && route_1hop.fidelity == IoFidelity::Lossy);

    // reentrancy guard predicate
    let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).expect("route resolves");
    check!("a plugin owning neither hop is safe", route_reenters_calling_plugin(&graph, &route, "norm").is_none());
    let hop = route_reenters_calling_plugin(&graph, &route, "stdio");
    check!("stdio owns (and is refused for) the first hop", hop == Some((&binary_raw, &gif_87a)));
    let hop = route_reenters_calling_plugin(&graph, &route, "gif");
    check!("gif owns (and is refused for) the second hop", hop.map(|h| h.1) == Some(&gif_89a));

    // conflict preflight
    let single = build_graph(&[("stdio", IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: IoFidelity::Exact, sniffs: true })]);
    let reclaim = vec![IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: IoFidelity::Exact, sniffs: true }];
    check!("same plugin reclaiming its own key is not a conflict", io_entries_conflict(&single, "stdio", &reclaim).is_none());
    let steal = vec![IoEntryDescriptor { from: binary_raw, into: gif_87a, fidelity: IoFidelity::Lossy, sniffs: false }];
    check!("a different plugin claiming the same key is a conflict", io_entries_conflict(&single, "gif", &steal).is_some());

    println!();
    if failures > 0 {
        println!("{failures} check(s) FAILED");
        std::process::exit(1);
    }
    println!("All checks passed");
}
