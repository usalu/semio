"""🗂️ G12 window-3 set (os-mcp host only): a hub workspace's capability catalog follows the hub's descriptor authority.

Root (H13, 2026-09-28): `HeadlessWorkspace::open_hub` compiled the capability `Catalog` ONCE from the startup snapshot, and
`RoutingArtifactChannel` held that same `Arc<Catalog>` — a gateway opened while the space held 4 packages never routed,
prepared or searched a 5th installed later ("cannot be matched to one of this workspace's 4 registered plugins").

The set (`🌉️mcp/🏠️workspace/🦀️.rs` + its quick tests):
  1. `WorkspaceCatalog`: `fixed(catalog)` for a folder; `following(hub)` for a hub — `current()` recompiles when the set of
     selected package descriptors (`plugin_id` + `descriptor_byte_sha256`) changes and otherwise reuses the compiled catalog;
     `authoritative()` fails closed while the authority refreshes (discovery), `current()` keeps routing on the last one.
  2. `HeadlessWorkspace.catalog` and `RoutingArtifactChannel.catalog` hold `Arc<WorkspaceCatalog>`; every read is `current()`;
     `discovery_catalog` of a hub reads `authoritative()` (no per-call recompile any more).
  3. The authenticated hub test fixture follows its binding's authority as `open_hub` does; law
     `a_hub_workspace_catalog_follows_a_new_descriptor_authority_generation`.

usage: python3 g12-live-catalog.py [--dry-run|--write] [--root <tree>]   (default --dry-run, root = the repo)
Idempotent: an applied tree reports "nothing to do"."""
import os
import sys

args = sys.argv[1:]
WRITE = "--write" in args
ROOT = args[args.index("--root") + 1] if "--root" in args else "/Users/ueli/Documents/semio"
MCP = os.path.join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp")
WORKSPACE = os.path.join(MCP, "🏠️workspace/🦀️.rs")
QUICK = os.path.join(MCP, "🏠️workspace/🧪️tests/🔬️quick/🦀️.rs")
APPLIED = "pub struct WorkspaceCatalog {"

WORKSPACE_CATALOG = '''/// 🗂️ The capability catalog a workspace routes, prepares and searches by. A folder workspace's is the one it opened
/// with; a hub workspace's follows the hub's descriptor authority — recompiled whenever the set of selected package
/// descriptors changes (a package installed, removed or upgraded on the hub), reused while that set is unchanged.
pub struct WorkspaceCatalog {
    hub: Option<Arc<HubRemoteBinding>>,
    compiled: std::sync::RwLock<(String, Arc<Catalog>)>,
}

impl WorkspaceCatalog {
    pub fn fixed(catalog: Arc<Catalog>) -> Self {
        Self { hub: None, compiled: std::sync::RwLock::new((String::new(), catalog)) }
    }

    /// 🌎️ The catalog of `hub`'s live descriptor authority, compiled from its selected packages now.
    pub fn following(hub: Arc<HubRemoteBinding>) -> Result<Self, GatewayError> {
        let snapshot = hub.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))?;
        let compiled = (selected_package_key(&snapshot), Arc::new(crate::catalog_from_descriptors(snapshot.selections.iter().map(|selection| selection.descriptor.clone()).collect())?));
        Ok(Self { hub: Some(hub), compiled: std::sync::RwLock::new(compiled) })
    }

    /// 🔒️ The catalog of the live authority, or its unavailability while it refreshes or after it was revoked — never a
    /// set the hub no longer selects. What discovery lists.
    pub fn authoritative(&self) -> Result<Arc<Catalog>, GatewayError> {
        let Some(hub) = &self.hub else { return Ok(Arc::clone(&self.compiled.read().unwrap_or_else(std::sync::PoisonError::into_inner).1)) };
        let snapshot = hub.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))?;
        let key = selected_package_key(&snapshot);
        let cached = {
            let compiled = self.compiled.read().unwrap_or_else(std::sync::PoisonError::into_inner);
            (compiled.0 == key).then(|| Arc::clone(&compiled.1))
        };
        if let Some(catalog) = cached {
            return Ok(catalog);
        }
        let catalog = Arc::new(crate::catalog_from_descriptors(snapshot.selections.iter().map(|selection| selection.descriptor.clone()).collect())?);
        *self.compiled.write().unwrap_or_else(std::sync::PoisonError::into_inner) = (key, Arc::clone(&catalog));
        Ok(catalog)
    }

    /// 🧭️ The catalog a command routes by: the live authority's, or the last compiled one while it refreshes.
    pub fn current(&self) -> Arc<Catalog> {
        self.authoritative().unwrap_or_else(|_| Arc::clone(&self.compiled.read().unwrap_or_else(std::sync::PoisonError::into_inner).1))
    }
}

/// 🔑️ Which package descriptors a catalog snapshot selects, order-free: each plugin with the byte hash of its descriptor.
fn selected_package_key(snapshot: &AuthorizedCatalogSnapshot) -> String {
    let mut packages: Vec<String> = snapshot.selections.iter().map(|selection| format!("{}@{}", selection.lease.package.plugin_id, selection.lease.package.descriptor_byte_sha256)).collect();
    packages.sort_unstable();
    packages.dedup();
    packages.join(",")
}

'''

WORKSPACE_EDITS = [
    ("RoutingArtifactChannel.catalog", "pub struct RoutingArtifactChannel {\n    catalog: Arc<Catalog>,\n", "pub struct RoutingArtifactChannel {\n    catalog: Arc<WorkspaceCatalog>,\n"),
    ("RoutingArtifactChannel::new", "    pub fn new(catalog: Arc<Catalog>, components: Option<PluginComponentSource>,", "    pub fn new(catalog: Arc<WorkspaceCatalog>, components: Option<PluginComponentSource>,"),
    (
        "RoutingArtifactChannel::route_for",
        """    fn route_for(&self, instance: u32, commands: &[AppCommand]) -> Result<AppRoute, Fault> {
        for command in commands {
            match command {
                AppCommand::PureCommand { capability_id, .. } => return resolve_route_for_capability_in(&self.catalog, capability_id).map_err(routing_fault),""",
        """    fn route_for(&self, instance: u32, commands: &[AppCommand]) -> Result<AppRoute, Fault> {
        let catalog = self.catalog.current();
        for command in commands {
            match command {
                AppCommand::PureCommand { capability_id, .. } => return resolve_route_for_capability_in(&catalog, capability_id).map_err(routing_fault),""",
    ),
    ("route_for slot", "        route_for_slot(&self.catalog, instance).ok_or_else(|| {", "        route_for_slot(&catalog, instance).ok_or_else(|| {"),
    ("route_for plugin count", "                    distinct_plugin_ids(&self.catalog).len()", "                    distinct_plugin_ids(&catalog).len()"),
    (
        "HeadlessWorkspace.catalog",
        "    repo_root: Option<PathBuf>,\n    catalog: Arc<Catalog>,\n",
        "    repo_root: Option<PathBuf>,\n    /// 🗂️ See [`WorkspaceCatalog`]: fixed for a folder, following the hub's descriptor authority for a hub.\n    catalog: Arc<WorkspaceCatalog>,\n",
    ),
    ("HeadlessWorkspace::new", "            repo_root: find_repo_root().ok(),\n            catalog,\n", "            repo_root: find_repo_root().ok(),\n            catalog: Arc::new(WorkspaceCatalog::fixed(catalog)),\n"),
    (
        "HeadlessWorkspace::open_hub",
        """            let descriptors = binding.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))?.selections.iter().map(|selection| selection.descriptor.clone()).collect();
            let catalog = Arc::new(crate::catalog_from_descriptors(descriptors)?);
            let driver = Arc::new(driver);
            let mut workspace = Self::new(WorkspaceOrigin::Hub { base_url, space_id }, principal, scopes, catalog);
""",
        """            let catalog = WorkspaceCatalog::following(Arc::clone(&binding))?;
            let driver = Arc::new(driver);
            let mut workspace = Self::new(WorkspaceOrigin::Hub { base_url, space_id }, principal, scopes, catalog.current());
            workspace.catalog = Arc::new(catalog);
""",
    ),
    (
        "discovery_catalog",
        """            WorkspaceOrigin::Hub { .. } => Ok(Arc::new(crate::catalog_from_descriptors(self.discovery_descriptors()?)?)),
            WorkspaceOrigin::Folder { .. } => Ok(self.catalog.clone()),""",
        """            WorkspaceOrigin::Hub { .. } => self.catalog.authoritative(),
            WorkspaceOrigin::Folder { .. } => Ok(self.catalog.current()),""",
    ),
    ("open_routing_channel", "        RoutingArtifactChannel::new(self.catalog.clone(), self.plugin_components(),", "        RoutingArtifactChannel::new(Arc::clone(&self.catalog), self.plugin_components(),"),
    ("search hits", "filter_map(|hit| self.catalog.get(&hit.capability_id)", "filter_map(|hit| catalog.get(&hit.capability_id)"),
    ("search", "        let hits = crate::search(&self.catalog, query, &filters);", "        let catalog = self.catalog.current();\n        let hits = crate::search(&catalog, query, &filters);"),
    ("describe", "        self.catalog.get(capability_id).map(|capability| serde_json::to_value(capability)", "        self.catalog.current().get(capability_id).map(|capability| serde_json::to_value(capability)"),
    (
        "prepare_action",
        """        let instance = capability_instance_slot(&self.catalog, capability_id).expect("a plugin capability of this workspace's own catalog always names a route of that catalog's own enumeration");
        let report = self.action_adapter()?.prepare(&self.catalog, """,
        """        let catalog = self.catalog.current();
        let instance = capability_instance_slot(&catalog, capability_id).expect("a plugin capability of this workspace's own catalog always names a route of that catalog's own enumeration");
        let report = self.action_adapter()?.prepare(&catalog, """,
    ),
]
BORROW_REPLACEMENT = ("&self.catalog,", "&self.catalog.current(),")
BORROW_REPLACEMENT_CLOSE = ("&self.catalog)", "&self.catalog.current())")
ANCHOR_BEFORE_ROUTING = "/// 🚦️ `crate::actions::ArtifactChannel` implementor that picks the plugin from the CALL instead of"

LAW_ANCHOR = "#[test]\nfn probe_pack_schema_hash_matches_the_cross_process_descriptor_contract() {"
LAW = '''/// 🗂️ A hub workspace's catalog follows the hub's descriptor authority: a new selection set (a package installed on the
/// hub) is compiled in, an unchanged set reuses the compiled catalog, and while the authority refreshes discovery fails
/// closed but routing keeps the last compiled catalog (measured 2026-09-28 by H13: a gateway opened on 4 packages never
/// saw the space's 5th).
#[test]
fn a_hub_workspace_catalog_follows_a_new_descriptor_authority_generation() {
    let workspace = authenticated_hub_workspace_fixture();
    let binding = Arc::clone(workspace.hub_binding.as_ref().expect("hub fixture"));
    let catalog = WorkspaceCatalog::following(Arc::clone(&binding)).expect("a ready authority compiles");
    let first = catalog.current();
    assert_eq!(distinct_plugin_ids(&first), vec!["gis".to_string()]);
    assert!(Arc::ptr_eq(&first, &catalog.current()), "an unchanged selection set is never recompiled");
    let mut selections = binding.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX)).expect("ready").selections.clone();
    let mut note = selections[0].clone();
    note.lease.package.plugin_id = "note".to_string();
    note.lease.package.descriptor_byte_sha256 = "note-descriptor".to_string();
    note.descriptor = load_package_descriptor(&find_repo_root().expect("repo root").join("✏️s/🔌️plugins/🗒️note")).expect("installed note descriptor test input");
    selections.push(note);
    binding.install_catalog_for_test(selections);
    let grown = catalog.current();
    assert_eq!(distinct_plugin_ids(&grown), vec!["gis".to_string(), "note".to_string()], "the package the hub selected since is routable");
    assert!(Arc::ptr_eq(&grown, &catalog.authoritative().expect("ready")), "discovery reads the same compiled catalog");
    binding.invalidate_stream();
    assert!(catalog.authoritative().is_err(), "a refreshing authority lists nothing");
    assert!(Arc::ptr_eq(&grown, &catalog.current()), "routing keeps the last compiled catalog while the authority refreshes");
}

'''

FIXTURE_OLD = "    workspace.hub_binding = Some(binding);\n    workspace\n}\n"
FIXTURE_NEW = "    workspace.catalog = Arc::new(WorkspaceCatalog::following(Arc::clone(&binding)).expect(\"the fixture's authority compiles\"));\n    workspace.hub_binding = Some(binding);\n    workspace\n}\n"
ROUTER_CTOR_OLD = "RoutingArtifactChannel::new(note_and_cad_catalog(), "
ROUTER_CTOR_NEW = "RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(note_and_cad_catalog())), "
ROUTER_CTOR_EMPTY_OLD = "RoutingArtifactChannel::new(empty_catalog(), "
ROUTER_CTOR_EMPTY_NEW = "RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(empty_catalog())), "
ROUTER_CTOR_CLONE_OLD = "RoutingArtifactChannel::new(Arc::clone(&catalog), "
ROUTER_CTOR_CLONE_NEW = "RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(Arc::clone(&catalog))), "


def main():
    problems, changed = [], {}
    workspace = open(WORKSPACE, encoding="utf-8").read()
    quick = open(QUICK, encoding="utf-8").read()
    if APPLIED in workspace:
        print("  workspace: applied")
    else:
        patched = workspace
        for name, old, new in WORKSPACE_EDITS:
            if patched.count(old) != 1:
                problems.append(f"workspace {name}: anchor count {patched.count(old)}")
                continue
            patched = patched.replace(old, new)
            print(f"  workspace {name}: insert")
        for old, new in (BORROW_REPLACEMENT, BORROW_REPLACEMENT_CLOSE):
            count = patched.count(old)
            patched = patched.replace(old, new)
            print(f"  workspace {old!r} → current(): {count}")
        patched = patched.replace("Arc::clone(&self.catalog.current())", "Arc::clone(&self.catalog)")
        if patched.count(ANCHOR_BEFORE_ROUTING) != 1:
            problems.append(f"workspace WorkspaceCatalog anchor count {patched.count(ANCHOR_BEFORE_ROUTING)}")
        else:
            patched = patched.replace(ANCHOR_BEFORE_ROUTING, WORKSPACE_CATALOG + ANCHOR_BEFORE_ROUTING)
        leftover = [line.strip() for line in patched.splitlines() if "self.catalog" in line and "self.catalog.current()" not in line and "self.catalog.authoritative()" not in line and "Arc::clone(&self.catalog)" not in line and "self.catalog_plugin_ids()" not in line]
        if leftover:
            problems.append(f"workspace: unconverted self.catalog reads: {leftover}")
        changed[WORKSPACE] = patched
    if "fn a_hub_workspace_catalog_follows_a_new_descriptor_authority_generation" in quick:
        print("  quick: applied")
    else:
        patched = quick
        for old, new in ((ROUTER_CTOR_OLD, ROUTER_CTOR_NEW), (ROUTER_CTOR_EMPTY_OLD, ROUTER_CTOR_EMPTY_NEW), (ROUTER_CTOR_CLONE_OLD, ROUTER_CTOR_CLONE_NEW)):
            print(f"  quick router ctor {old[26:48]!r}: {patched.count(old)}")
            patched = patched.replace(old, new)
        if patched.count(FIXTURE_OLD) != 1:
            problems.append(f"quick hub fixture anchor count {patched.count(FIXTURE_OLD)}")
        else:
            patched = patched.replace(FIXTURE_OLD, FIXTURE_NEW)
        if "RoutingArtifactChannel::new(" in patched.replace("RoutingArtifactChannel::new(Arc::new(WorkspaceCatalog::fixed(", ""):
            problems.append("quick: a RoutingArtifactChannel::new site is not converted")
        if patched.count(LAW_ANCHOR) != 1:
            problems.append(f"quick law anchor count {patched.count(LAW_ANCHOR)}")
        else:
            patched = patched.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)
        changed[QUICK] = patched
    print(f"root {ROOT}: {len(changed)} file(s) to change, {len(problems)} problem(s)")
    for problem in problems:
        print(f"  PROBLEM {problem}")
    if problems:
        sys.exit(1)
    if WRITE:
        for path, text in changed.items():
            open(path, "w", encoding="utf-8").write(text)
        print("written; gate: zsh wp-g12/g12-gate.sh <tag> (check --lib --tests, quick, gateway build)")
    else:
        print("dry run clean" if changed else "nothing to do (applied)")


main()
