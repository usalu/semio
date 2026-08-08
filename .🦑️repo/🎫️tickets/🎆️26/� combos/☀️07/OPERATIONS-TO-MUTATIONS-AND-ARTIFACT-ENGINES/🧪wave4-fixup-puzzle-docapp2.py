#!/usr/bin/env python3
"""Finish puzzle 3d/5d DocumentApp static migration: TLS session + signature fixes."""

from __future__ import annotations

from pathlib import Path
import re


def find_apps() -> tuple[Path, Path]:
    root = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
    puzzle = next(p for p in root.iterdir() if "puzzle" in p.name)
    apps = puzzle / "🎛️apps"
    app3d = next(p for p in apps.iterdir() if "3d" in p.name) / "🦀️component.rs"
    app5d = next(p for p in apps.iterdir() if "5d" in p.name) / "🦀️component.rs"
    return app3d, app5d


def strip_app_id_document_schema(block: str) -> str:
    block = re.sub(
        r"\n    fn app_id\(&self\) -> &str \{\n        [^\n]+\n    \}\n",
        "\n",
        block,
    )
    block = re.sub(
        r"\n    fn document_schema\(&self\) -> &str \{\n        [^\n]+\n    \}\n",
        "\n",
        block,
    )
    return block


def fix_handle_sig(block: str, command: str, projection: str, config: str, mutation: str, config_mutation: str) -> str:
    # Tolerate optional space after (
    pattern = (
        rf"fn handle\(\s*command: &{command}, doc: &DocumentView<'_, {projection}>, "
        rf"cfg: &ConfigView<'_, {config}>\) -> Result<Emit<{mutation}, {config_mutation}>, Fault>"
    )
    repl = (
        f"fn handle(command: &{command}, doc: &DocumentView<'_, {projection}>, "
        f"cfg: &ConfigView<'_, {config}>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) "
        f"-> Result<Emit<{mutation}, {config_mutation}, Self::DraftMutation>, Fault>"
    )
    block2, n = re.subn(pattern, repl, block)
    print(f"  handle replacements: {n}")
    block2 = block2.replace(
        f"Result<Emit<{mutation}, {config_mutation}>, MediaError>",
        f"Result<Emit<{mutation}, {config_mutation}, Self::DraftMutation>, MediaError>",
    )
    block2 = re.sub(
        rf"fn command_id\(\s*command: &{command}\) -> &str",
        f"fn command_id(command: &{command}) -> &'static str",
        block2,
    )
    return block2


def inject_tls_3d(text: str) -> str:
    if "PUZZLE3D_PLAY_SESSION" in text:
        return text
    needle = "pub struct Puzzle3dPlayApp {"
    insert = (
        "thread_local! {\n"
        "    /// 🧠 Long-lived play session — `DocumentApp` methods are associated fns (no `&self`),\n"
        "    /// so the precompute/gumball scratch lives here until `EngineHandles` carries it.\n"
        "    static PUZZLE3D_PLAY_SESSION: std::cell::RefCell<Puzzle3dPlayApp> = std::cell::RefCell::new(Puzzle3dPlayApp::default());\n"
        "}\n\n"
        "fn with_puzzle3d_app<R>(f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {\n"
        "    PUZZLE3D_PLAY_SESSION.with(|app| f(&app.borrow()))\n"
        "}\n\n"
        "fn with_puzzle3d_app_mut<R>(f: impl FnOnce(&Puzzle3dPlayApp) -> R) -> R {\n"
        "    PUZZLE3D_PLAY_SESSION.with(|app| f(&app.borrow()))\n"
        "}\n\n"
    )
    return text.replace(needle, insert + needle, 1)


def inject_tls_5d(text: str) -> str:
    if "PUZZLE5D_PLAY_SESSION" in text:
        return text
    needle = "pub struct Puzzle5dPlayApp {"
    insert = (
        "thread_local! {\n"
        "    /// 🧠 Long-lived play session — `DocumentApp` methods are associated fns (no `&self`),\n"
        "    /// so the precompute session lives here until `EngineHandles` carries it.\n"
        "    static PUZZLE5D_PLAY_SESSION: RefCell<Puzzle5dPlayApp> = RefCell::new(Puzzle5dPlayApp::default());\n"
        "}\n\n"
        "fn with_puzzle5d_app<R>(f: impl FnOnce(&Puzzle5dPlayApp) -> R) -> R {\n"
        "    PUZZLE5D_PLAY_SESSION.with(|app| f(&app.borrow()))\n"
        "}\n\n"
    )
    return text.replace(needle, insert + needle, 1)


def wrap_docapp_self_methods_3d(block: str) -> str:
    """Rewrite DocumentApp bodies that still reference self.* to use with_puzzle3d_app."""
    # handle_action_impl is outside DocumentApp - handled separately
    # For methods that still contain self. — wrap entire body
    # Simpler targeted replacements for known patterns in render/window_*/tool_*/handle:

    # handle body: Self::handle_action_impl already — but handle_action_impl needs app
    # Change handle to:
    # Ok(with_puzzle3d_app(|app| app.handle_action_impl(...)))
    block = re.sub(
        r"Ok\(Self::handle_action_impl\(([^)]+)\)\)",
        r"Ok(with_puzzle3d_app(|app| app.handle_action_impl(\1)))",
        block,
        count=1,
    )

    # For render / window_engagements / window_measures / tool_measures / import_media that use self.
    # Wrap any method body that still has self. 
    def wrap_method(match: re.Match[str]) -> str:
        header, body = match.group(1), match.group(2)
        if "self." not in body and "self," not in body and re.search(r"\bself\b", body) is None:
            return match.group(0)
        # indent body one more level inside closure
        indented = "\n".join(
            ("            " + line if line.strip() else line) for line in body.splitlines()
        )
        # if body starts with newline after {
        return f"{header} {{\n        with_puzzle3d_app(|app| {{\n{indented}\n        }})\n    }}"

    # Only wrap methods that still reference self
    for meth in ["render", "window_engagements", "window_measures", "tool_measures", "import_media", "context_menu"]:
        pattern = rf"(fn {meth}\([^\)]*\)[^{{]*\{{)(.*?)(\n    \}})"
        # manual find
        m = re.search(rf"fn {meth}\(", block)
        if not m:
            continue
        # find body braces
        start = block.find("{", m.start())
        depth = 0
        end = None
        for j in range(start, len(block)):
            if block[j] == "{":
                depth += 1
            elif block[j] == "}":
                depth -= 1
                if depth == 0:
                    end = j
                    break
        if end is None:
            continue
        header = block[m.start() : start]
        body = block[start + 1 : end]
        if not re.search(r"\bself\b", body):
            continue
        # replace self. with app. and bare self with app where needed
        new_body = re.sub(r"\bself\.", "app.", body)
        new_body = re.sub(r"\bself\b", "app", new_body)
        indented = "".join(
            ("            " + line if line.strip() else line) for line in new_body.splitlines(True)
        )
        # Detect return type - if UiNode / HashMap / Result / Option, closure must return that
        replacement = f"{header} {{\n        with_puzzle3d_app(|app| {{{indented}        }})\n    }}"
        block = block[: m.start()] + replacement + block[end + 1 :]
        print(f"  wrapped {meth}")
    return block


def fix_handle_action_impl_3d(text: str) -> str:
    """Restore &self on handle_action_impl (instance method on PlayApp)."""
    text2, n = re.subn(
        r"fn handle_action_impl\(\s*action:",
        "fn handle_action_impl(&self, action:",
        text,
        count=1,
    )
    print(f"  restore handle_action_impl &self: {n}")
    return text2


def fix_handle_action_impl_5d(text: str) -> str:
    text2, n = re.subn(
        r"fn handle_action_impl\(\s*action:",
        "fn handle_action_impl(&self, action:",
        text,
        count=1,
    )
    print(f"  restore handle_action_impl &self: {n}")
    # Fix `app: self` if somehow became broken — should be fine with &self back
    return text2


def wrap_docapp_self_methods_5d(block: str) -> str:
    block = re.sub(
        r"Ok\(Self::handle_action_impl\(([^)]+)\)\)",
        r"Ok(with_puzzle5d_app(|app| app.handle_action_impl(\1)))",
        block,
        count=1,
    )
    for meth in ["render", "window_engagements", "window_measures", "tool_measures", "import_media", "context_menu"]:
        m = re.search(rf"fn {meth}\(", block)
        if not m:
            continue
        start = block.find("{", m.start())
        depth = 0
        end = None
        for j in range(start, len(block)):
            if block[j] == "{":
                depth += 1
            elif block[j] == "}":
                depth -= 1
                if depth == 0:
                    end = j
                    break
        if end is None:
            continue
        header = block[m.start() : start]
        body = block[start + 1 : end]
        if not re.search(r"\bself\b", body):
            continue
        new_body = re.sub(r"\bself\.", "app.", body)
        new_body = re.sub(r"\bself\b", "app", new_body)
        indented = "".join(
            ("            " + line if line.strip() else line) for line in new_body.splitlines(True)
        )
        replacement = f"{header} {{\n        with_puzzle5d_app(|app| {{{indented}        }})\n    }}"
        block = block[: m.start()] + replacement + block[end + 1 :]
        print(f"  wrapped {meth}")
    return block


def patch_file(path: Path, dim: str) -> None:
    print("====", path)
    text = path.read_text()
    if dim == "3d":
        text = inject_tls_3d(text)
        text = fix_handle_action_impl_3d(text)
        command, projection, config = "Puzzle3dCommand", "Puzzle3dPlayProjection", "Puzzle3dConfig"
        mutation, config_mutation = "Puzzle3dMutation", "Puzzle3dConfigMutation"
        wrap = wrap_docapp_self_methods_3d
    else:
        text = inject_tls_5d(text)
        text = fix_handle_action_impl_5d(text)
        command, projection, config = "Puzzle5dCommand", "Puzzle5dPlayProjection", "Puzzle5dConfig"
        mutation, config_mutation = "Puzzle5dMutation", "Puzzle5dConfigMutation"
        wrap = wrap_docapp_self_methods_5d

    start = text.find(f"impl DocumentApp for Puzzle{dim.upper()}PlayApp") if False else None
    # Use exact names
    app = "Puzzle3dPlayApp" if dim == "3d" else "Puzzle5dPlayApp"
    start = text.find(f"impl DocumentApp for {app} {{")
    create = text.find("\npub fn create_", start)
    block = text[start:create]
    block = strip_app_id_document_schema(block)
    block = fix_handle_sig(block, command, projection, config, mutation, config_mutation)
    block = wrap(block)
    text = text[:start] + block + text[create:]
    path.write_text(text)
    print("wrote", path)


def main() -> None:
    app3d, app5d = find_apps()
    patch_file(app3d, "3d")
    patch_file(app5d, "5d")


if __name__ == "__main__":
    main()
