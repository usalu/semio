"""🧪 Run the exact Rust pending-effect poll calls against an owner that refuses temporary clones."""
import re
import subprocess
import tempfile
from pathlib import Path

repository = Path(__file__).resolve().parents[7]
ops = repository / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops"
background = (ops / "🧾️typed-operations/🦀️.rs").read_text(encoding="utf-8")
refresh = (ops / "🖥️surface-render/🦀️.rs").read_text(encoding="utf-8")
marker = "pub fn plugin_drive_pending_effects"
background = background[background.index(marker):]
background = re.search(r"let active = .*?\n.*?\n        match ([^\n]+)\? \{", background)
assert background
background = background[0].replace(f"match {background[1]}? {{", f"let polled = {background[1]}?;")
refresh = re.search(r"let polled = \{\n\s*let instance = &mut \*instance;\n.*?pending_effects[^\n]+\n\s*\};", refresh)
assert refresh
refresh = refresh[0]
baseline_background = background.replace("let active = &mut *active;\n        let view = active.surface_contexts.view();", "let view = active.surface_contexts.view().cloned();").replace("pending_effects(view, cx)", "pending_effects(view.as_ref(), cx)")
baseline_refresh = refresh.replace("let instance = &mut *instance;", "let registered_view = instance.surface_contexts.view().cloned();").replace("pending_effects(instance.surface_contexts.view(), cx)", "pending_effects(registered_view.as_ref(), cx)")
source = r"""
use std::sync::Mutex;
extern crate self as semio_framework_async;

pub mod poll {
    pub fn resolve_ready<F: std::future::Future>(fut: F) -> F::Output {
        let cx = &mut std::task::Context::from_waker(std::task::Waker::noop());
        match std::pin::pin!(fut).poll(cx) {
            std::task::Poll::Ready(value) => value,
            _ => panic!("fixture unexpectedly suspended")
        }
    }
}

struct View {
    utility: &'static str,
    duplicate: bool,
}

impl Clone for View {
    fn clone(&self) -> Self {
        Self { utility: self.utility, duplicate: true }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        assert!(!self.duplicate, "poll dropped an unretired temporary view");
    }
}

struct Surfaces(View);
impl Surfaces {
    fn view(&self) -> Option<&View> { Some(&self.0) }
}

struct App(usize);
impl App {
    async fn pending_effects(&mut self, view: Option<&View>, _: &mut ()) -> Result<Option<&'static str>, ()> {
        let utility = view.unwrap().utility;
        self.0 += 1;
        Ok((self.0 == 2).then_some(utility))
    }
}

struct Instance {
    app: App,
    surface_contexts: Surfaces,
}

fn background(cell: &Mutex<Instance>, cx: &mut ()) -> Result<Option<&'static str>, ()> {
    let mut active = cell.lock().unwrap();
    BACKGROUND
    Ok(polled)
}

fn refresh(cell: &Mutex<Instance>, cx: &mut ()) -> Result<Option<&'static str>, ()> {
    let mut instance = cell.lock().unwrap();
    REFRESH
    Ok(polled)
}

fn main() {
    let lane = match std::env::args().nth(1).as_deref() {
        Some("background") => background,
        Some("refresh") => refresh,
        _ => unreachable!(),
    };
    let cell = Mutex::new(Instance {
        app: App(0),
        surface_contexts: Surfaces(View { utility: "pen", duplicate: false }),
    });
    assert_eq!(lane(&cell, &mut ()), Ok(None));
    assert_eq!(lane(&cell, &mut ()), Ok(Some("pen")));
    assert_eq!(cell.lock().unwrap().surface_contexts.0.utility, "pen");
}
"""

with tempfile.TemporaryDirectory(dir=Path(__file__).parent / "🗑️generated") as directory:
    binary = Path(directory).resolve() / "pending-effect-borrow"
    for baseline, a, b in [(True, baseline_background, baseline_refresh), (False, background, refresh)]:
        assembled = source.replace("BACKGROUND", a).replace("REFRESH", b)
        subprocess.run(["repo", "tools", "exec", "--light", "rustc", "--", "--edition=2021", "-Awarnings", "-o", str(binary), "-"], input=assembled, text=True, check=True)
        for lane in ("background", "refresh"):
            run = subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(binary), lane], capture_output=True, text=True, timeout=45)
            if baseline:
                assert run.returncode != 0 and "poll dropped an unretired temporary view" in run.stderr, run
            else:
                assert run.returncode == 0, run
            print(lane, "baseline" if baseline else "fixed", run.returncode)
