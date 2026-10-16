"""Generates the test-support module `📜️repo-text` (repository text readers and the five source sets)."""
import json, os
from lib import *
lists = json.load(open(SCRATCH + "/source-lists.json"))
crates = crate_table()
order = ["shell-vocab","shell-async","shell-chrome-state","shell-surfaces","shell-plugins","shell-history","shell-prefs","shell-directory","shell-sync","shell-docs","shell-doc-open","shell-windows","shell-dock","shell-input","shell-commands","shell-state","shell-session","shell-window-actions","shell-actions","shell-pointer","shell-view-paint","shell-view-chrome","shell-view-overlays","shell-view-panels"]
engine_rel = os.path.relpath(E, REPO)
def lit(items, indent="    "):
    return "\n".join(f'{indent}"{i}",' for i in items)
shell_roots = [os.path.relpath(SH + "/🦀️.rs", E)] + ["🎯️targets/🧊️wgpu/" + crates[c] + "/🦀️.rs" for c in order]
for r in shell_roots:
    assert os.path.exists(E + "/" + r), r
text = f'''//! 📜️ Repository text readers of the renderer source-text laws: run-time reads replace the `include_str!` embeds of production sources and the `*_SOURCE` consts.
//!
//! 🧭️ Every path is relative to the repository root (`repo_text`, `repo_texts`) or to the renderer engine folder (the source sets); the root is found by walking up from this crate's manifest, so a reader works from any test crate of the renderer. Texts are read once and kept for the process.
//! @see <https://doc.rust-lang.org/std/macro.include_str.html>

use std::collections::HashMap;
use std::path::{{Path, PathBuf}};
use std::sync::{{Mutex, OnceLock}};

use crate::module_tree::read_module_tree_source;

const ENGINE: &str = "{engine_rel}";

/// 🧾️ The files the renderer root, its hubs and the crates they mount held as one text (the former `RENDERER_SOURCE`), relative to the engine folder.
pub const RENDERER_SOURCE_FILES: &[&str] = &[
{lit(lists["renderer"])}
];

/// 🧾️ The files of the Scenes facade held as one text (the former `SCENES_SOURCE`).
pub const SCENES_SOURCE_FILES: &[&str] = &[
{lit(lists["scenes"])}
];

/// 🧾️ The files of the EngineCanvas facade held as one text (the former `ENGINE_CANVAS_SOURCE`).
pub const ENGINE_CANVAS_SOURCE_FILES: &[&str] = &[
{lit(lists["engine_canvas"])}
];

/// 🧾️ The files of the Interpreter facade held as one text (the former `INTERPRETER_SOURCE`).
pub const INTERPRETER_SOURCE_FILES: &[&str] = &[
{lit(lists["interpreter"])}
];

/// 🐚️ The roots of the Shell module tree: the `shell` parent and the library root of every Shell crate (the former `SHELL_SOURCE`), relative to the engine folder.
pub const SHELL_SOURCE_ROOTS: &[&str] = &[
{lit(shell_roots)}
];

type Cache = Mutex<HashMap<String, &'static str>>;

fn cache() -> &'static Cache {{
    static CACHE: OnceLock<Cache> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}}

fn remember(key: String, build: impl FnOnce() -> String) -> &'static str {{
    if let Some(text) = cache().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&key) {{
        return text;
    }}
    let leaked: &'static str = Box::leak(build().into_boxed_str());
    cache().lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(key).or_insert(leaked)
}}

/// 🗺️ The repository root: the first ancestor of this crate's manifest that holds the framework folder and a workspace manifest.
pub fn repo_root() -> &'static Path {{
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {{
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|ancestor| ancestor.join("Cargo.toml").is_file() && ancestor.join("{engine_rel.split('/')[0]}").is_dir())
            .expect("a repository root above the test-support manifest")
            .to_path_buf()
    }})
}}

/// 🧑‍🎨 The renderer engine folder (`…/📺️renderer/🧑‍🎨engine`).
pub fn engine_root() -> PathBuf {{
    repo_root().join(ENGINE)
}}

/// 📖️ One repository file as text, read once; a missing file names its path in the panic.
pub fn repo_text(relative: &str) -> &'static str {{
    remember(format!("file:{{relative}}"), || {{
        let path = repo_root().join(relative);
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{{}} is readable: {{error}}", path.display()))
    }})
}}

/// 📚️ Several repository files as one text, in order, each newline-terminated.
pub fn repo_texts(relatives: &[&str]) -> &'static str {{
    remember(format!("files:{{}}", relatives.join("\\n")), || {{
        relatives.iter().map(|relative| {{
            let text = repo_text(relative);
            if text.ends_with('\\n') {{ text.to_owned() }} else {{ format!("{{text}}\\n") }}
        }}).collect()
    }})
}}

fn engine_texts(relatives: &[&str]) -> &'static str {{
    remember(format!("engine:{{}}", relatives.join("\\n")), || {{
        let full: Vec<String> = relatives.iter().map(|relative| format!("{{ENGINE}}/{{relative}}")).collect();
        let refs: Vec<&str> = full.iter().map(String::as_str).collect();
        repo_texts(&refs).to_owned()
    }})
}}

/// 🔎️ The renderer root, its hubs and the crates they mount as one text.
pub fn renderer_source() -> &'static str {{
    engine_texts(RENDERER_SOURCE_FILES)
}}

/// 🔎️ The Scenes facade as one text.
pub fn scenes_source() -> &'static str {{
    engine_texts(SCENES_SOURCE_FILES)
}}

/// 🔎️ The EngineCanvas facade as one text.
pub fn engine_canvas_source() -> &'static str {{
    engine_texts(ENGINE_CANVAS_SOURCE_FILES)
}}

/// 🔎️ The Interpreter facade as one text.
pub fn interpreter_source() -> &'static str {{
    engine_texts(INTERPRETER_SOURCE_FILES)
}}

/// 🔎️ The whole Shell module tree (the `shell` parent and every Shell crate) as one text, `🧪️tests` mounts excluded.
pub fn shell_source() -> &'static str {{
    remember("shell".to_owned(), || {{
        SHELL_SOURCE_ROOTS.iter().map(|root| read_module_tree_source(&engine_root().join(root)).expect("the shell source tree is readable")).collect()
    }})
}}
'''
out = T + "/🧯️test-support/📜️repo-text/🦀️.rs"
write(out, text)
print(len(text.split("\n")), "lines", out)
