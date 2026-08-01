from pathlib import Path
import re

base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugin")
puzzle = [x for x in base.iterdir() if "puzzle" in x.name][0]


def ensure_refcell_import(t: str) -> str:
    if "use std::cell::RefCell" in t or "std::cell::RefCell" in t:
        return t
    if "use std::collections::" in t:
        return t.replace("use std::collections::", "use std::cell::RefCell;\nuse std::collections::", 1)
    return "use std::cell::RefCell;\n" + t


def wrap_field_access(t: str, field: str) -> str:
    # self.field.foo -> self.field.borrow_mut().foo  (skip if already .borrow)
    return re.sub(rf"self\.{field}\.(?!borrow)", rf"self.{field}.borrow_mut().", t)


def migrate_2d(path: Path) -> None:
    t = ensure_refcell_import(path.read_text())
    old_struct = """pub struct Puzzle2dPlayApp {
    host: BoardHost,
    runtime: Puzzle2dPlayRuntime,
    /// 🗄️ The fixture content last parsed into `host` via `parse_fixture_v1` — lets `handle_action`
    /// skip that full clear-scene-and-rebuild (and the kind-catalog/kind-compat re-push) on the
    /// large majority of actions (select/camera/utility/…) that never touch fixture content.
    last_synced_fixture: Option<Value>,
}"""
    new_struct = """pub struct Puzzle2dPlayApp {
    host: RefCell<BoardHost>,
    runtime: RefCell<Puzzle2dPlayRuntime>,
    /// 🗄️ The fixture content last parsed into `host` via `parse_fixture_v1` — lets `handle_action`
    /// skip that full clear-scene-and-rebuild (and the kind-catalog/kind-compat re-push) on the
    /// large majority of actions (select/camera/utility/…) that never touch fixture content.
    last_synced_fixture: RefCell<Option<Value>>,
}"""
    if old_struct not in t:
        raise SystemExit("2d struct not found")
    t = t.replace(old_struct, new_struct)
    t = t.replace(
        "Self { host: puzzle_board_host(), runtime: Puzzle2dPlayRuntime::default(), last_synced_fixture: None }",
        "Self { host: RefCell::new(puzzle_board_host()), runtime: RefCell::new(Puzzle2dPlayRuntime::default()), last_synced_fixture: RefCell::new(None) }",
    )
    t = t.replace("&mut self.host", "&mut self.host.borrow_mut()")
    t = t.replace("self.last_synced_fixture = ", "*self.last_synced_fixture.borrow_mut() = ")
    t = t.replace("self.last_synced_fixture.as_ref()", "self.last_synced_fixture.borrow().as_ref()")
    t = t.replace("runtime: self.runtime.clone()", "runtime: self.runtime.borrow().clone()")
    t = t.replace("self.runtime = ", "*self.runtime.borrow_mut() = ")
    t = wrap_field_access(t, "runtime")
    t = wrap_field_access(t, "host")
    # cleanup doubled
    t = t.replace("self.host.borrow_mut().borrow_mut()", "self.host.borrow_mut()")
    t = t.replace("self.runtime.borrow_mut().borrow_mut()", "self.runtime.borrow_mut()")
    t = t.replace("self.runtime.borrow().borrow_mut()", "self.runtime.borrow_mut()")
    t = t.replace("&mut self.host.borrow_mut().borrow_mut()", "&mut self.host.borrow_mut()")
    path.write_text(t)
    print("2d ok")


def migrate_5d(path: Path) -> None:
    t = ensure_refcell_import(path.read_text())
    old = """pub struct Puzzle5dPlayApp {
    precompute: Puzzle5dPrecomputeSession,
    registered_mesh_urls: HashSet<String>,
    runtime: Puzzle5dRuntime,
}"""
    new = """pub struct Puzzle5dPlayApp {
    precompute: RefCell<Puzzle5dPrecomputeSession>,
    registered_mesh_urls: RefCell<HashSet<String>>,
    runtime: RefCell<Puzzle5dRuntime>,
}"""
    if old not in t:
        m = re.search(r"pub struct Puzzle5dPlayApp \{[\s\S]*?\n\}", t)
        raise SystemExit("5d struct not found: " + (m.group(0) if m else "none"))
    t = t.replace(old, new)
    t = t.replace(
        "precompute: Puzzle5dPrecomputeSession::new(), registered_mesh_urls: HashSet::new(), runtime: Puzzle5dRuntime::default()",
        "precompute: RefCell::new(Puzzle5dPrecomputeSession::new()), registered_mesh_urls: RefCell::new(HashSet::new()), runtime: RefCell::new(Puzzle5dRuntime::default())",
    )
    t = t.replace("self.runtime = ", "*self.runtime.borrow_mut() = ")
    t = t.replace("self.registered_mesh_urls = ", "*self.registered_mesh_urls.borrow_mut() = ")
    t = t.replace("&mut self.precompute", "&mut self.precompute.borrow_mut()")
    t = t.replace("&mut self.runtime", "&mut self.runtime.borrow_mut()")
    t = t.replace("&mut self.registered_mesh_urls", "&mut self.registered_mesh_urls.borrow_mut()")
    t = t.replace("&self.precompute", "&self.precompute.borrow()")
    t = t.replace("&self.runtime", "&self.runtime.borrow()")
    t = t.replace("&self.registered_mesh_urls", "&self.registered_mesh_urls.borrow()")
    for field in ("runtime", "precompute", "registered_mesh_urls"):
        t = wrap_field_access(t, field)
    for field in ("runtime", "precompute", "registered_mesh_urls"):
        t = t.replace(f"self.{field}.borrow_mut().borrow_mut()", f"self.{field}.borrow_mut()")
        t = t.replace(f"self.{field}.borrow().borrow_mut()", f"self.{field}.borrow_mut()")
        t = t.replace(f"&mut self.{field}.borrow_mut().borrow_mut()", f"&mut self.{field}.borrow_mut()")
        t = t.replace(f"&self.{field}.borrow().borrow()", f"&self.{field}.borrow()")
    path.write_text(t)
    print("5d ok")


d2 = [x for x in (puzzle / "🎛️app").iterdir() if "2d" in x.name][0]
d5 = [x for x in (puzzle / "🎛️app").iterdir() if "5d" in x.name][0]
migrate_2d(d2 / "🔨️module" / "🖱️ui" / "⚡️implementation" / "🦀️rust" / "📦️lib.rs")
migrate_5d(d5 / "🔨️module" / "🖱️ui" / "⚡️implementation" / "🦀️rust" / "📦️lib.rs")
