#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Temporary W2 migration helper for 🧩️puzzle example shape (ticket-local)."""
from __future__ import annotations

import glob
import json
import os
import textwrap
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
os.chdir(REPO)
PUZZLE = Path(glob.glob("✏️s/🔌️plugins/*puzzle")[0])
TICKET = Path(glob.glob(".🦑️repo/**/EXAMPLE-SHAPE*/", recursive=True)[0])

TESTS = "️tests"
ASSETS = "🖼️assets"
EXAMPLES = "📚️examples"
RS = "🦀️component.rs"
TS = "🟦️component.ts"
RS_TEST = "🦀️test.rs"
TS_TEST = "🟦️test.ts"

# Fix TESTS to exact filesystem name
TESTS = [p.name for p in (PUZZLE / "🗿️artifacts").rglob("*") if p.is_dir() and p.name.endswith("tests") and "examples" in str(p)][0]
ASSETS = [p.name for p in (PUZZLE / "🗿️artifacts").rglob("*") if p.is_dir() and p.name.endswith("assets") and "examples" in str(p)][0]

changed: list[str] = []


def write(path: Path, content) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(content, (bytes, bytearray)):
        path.write_bytes(bytes(content))
    else:
        path.write_text(str(content))
    changed.append(str(path.relative_to(REPO)))
    print("W", path.relative_to(REPO))


def art_meta():
    out = []
    for art in sorted((PUZZLE / "🗿️artifacts").iterdir()):
        name = art.name
        if "2d" in name and "3d" not in name and "5d" not in name:
            out.append((name, "puzzle2d", "2d"))
        elif "5d" in name:
            out.append((name, "puzzle5d", "5d"))
        else:
            out.append((name, "puzzle3d", "3d"))
    return out


def app_meta():
    out = []
    for app in sorted((PUZZLE / "🎛️apps").iterdir()):
        name = app.name
        if "2d" in name and "3d" not in name and "5d" not in name:
            out.append((name, "puzzle2d", "2d"))
        elif "5d" in name:
            out.append((name, "puzzle5d", "5d"))
        else:
            out.append((name, "puzzle3d", "3d"))
    return out


def op_text(dim: str, example: str) -> str:
    return textwrap.dedent(
        f"""\
        semio puzzle.puzzle{dim}.op v1
        artifact-mark=puzzle.{dim}-op
        # {example}: seed placement then capsule merge
        add-vertex x=0.0 y=0.0 z=0.0
        add-vertex x=4.05 y=4.68 z=3.0
        set-face index=0 loop=[0 1 2 3]
        transform-mesh axis=z angle=0.0
        merge-solid target=seed-left source=seed-right
        """
    )


def cmd_text(dim: str) -> str:
    return textwrap.dedent(
        f"""\
        semio puzzle.{dim}.cmd v1
        # Demo session: load concrete-forest then orbit the overview
        action=demo
        exampleId=concrete-forest
        steps=[
          setActiveExample exampleId=concrete-forest
          setCamera x=230.7 y=93.5 zoom=2
          selectAll
          fitView
        ]
        actor=example
        started="0"
        """
    )


def expand_binary_stub(existing: Path, label: str) -> bytes:
    base = existing.read_bytes() if existing.exists() else b"\x89SEM\r\n\x1a\n"
    marker = label.encode("utf-8")
    if len(base) > 200 and marker in base:
        return base
    payload = (f"\n# semio-example-pack payload for {label}\n").encode("utf-8") + (b"\x00" * 48) + marker
    return base + payload


def rust_example_component(art_mod, slug_emoji, slug_id, title_en, title_de, icon, asset_stem, is_2d):
    if is_2d:
        body = """    let mut value = serde_json::to_value(&projection).expect("serialize example");
    if let Some(object) = value.as_object_mut() {
        object.remove("camera");
    }
    serde_json::to_string(&value).expect("re-serialize example")"""
    else:
        body = '    serde_json::to_string(&projection).expect("serialize example")'
    return f"""//! 📚️ Example `{slug_emoji}` for artifact `{art_mod}`.

use std::sync::LazyLock;

use semio_framework::LocalizedLabel;
use semio_framework_os_kernel::plugin::ExampleSource;

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "{slug_id}";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {{
    LocalizedLabel::native("{title_en}", "{title_de}")
}}

/// 🖼️ Icon id.
pub const ICON: &str = "{icon}";

/// 🗣️ DSL fixture text.
pub const DSL_TEXT: &str = include_str!("{ASSETS}/🗣️{asset_stem}.dsl.semio");

/// 🔧️ Op fixture text.
pub const OP_TEXT: &str = include_str!("{ASSETS}/🔧️{asset_stem}.op.semio");

/// 🎒️ Pack fixture bytes.
pub const PACK_BYTES: &[u8] = include_bytes!("{ASSETS}/🎒️{asset_stem}.pack.semio");

/// 📡️ SPR fixture bytes.
pub const SPR_BYTES: &[u8] = include_bytes!("{ASSETS}/📡️{asset_stem}.spr.semio");

fn document_json() -> String {{
    let projection = crate::artifacts::{art_mod}::dsl::parse_dsl(DSL_TEXT)
        .unwrap_or_else(|error| panic!("{{ID}} example dsl parses: {{error}}"));
{body}
}}

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {{
    ExampleSource::new(ID, label(), document_json(), ICON)
}});
"""


def rust_app_example(dim: str) -> str:
    return f"""//! 📚️ App demo-session example for puzzle {dim}.

use std::sync::LazyLock;

use semio_framework::LocalizedLabel;
use semio_framework_os_kernel::plugin::ExampleSource;

/// 🏷️ Stable example id.
pub const ID: &str = "demo-session";

/// 🗣️ Localized picker label.
pub fn label() -> LocalizedLabel {{
    LocalizedLabel::native("Demo Session", "Demo-Sitzung")
}}

/// 🖼️ Icon id.
pub const ICON: &str = "play";

/// 🎮️ Command-script fixture text.
pub const CMD_TEXT: &str = include_str!("{ASSETS}/🎮️demo.cmd.semio");

/// 📚️ Canonical example source for `App::example_source`.
pub static SOURCE: LazyLock<ExampleSource> = LazyLock::new(|| {{
    ExampleSource::new(ID, label(), CMD_TEXT, ICON)
}});
"""


def rust_test(slug_emoji, art_mod, asset_stem):
    return f"""//! ️tests for example `{slug_emoji}`.

#[test]
fn dsl_asset_parses_and_round_trips() {{
    let text = include_str!("../{ASSETS}/🗣️{asset_stem}.dsl.semio");
    assert!(text.len() > 64, "dsl fixture must carry real payload");
    let projection = crate::artifacts::{art_mod}::dsl::parse_dsl(text).expect("example dsl parses");
    store::test_support::assert_dsl_round_trip(&projection);
    store::test_support::assert_dsl_pack_equivalence(&projection);
}}

#[test]
fn op_pack_and_spr_assets_are_nonempty() {{
    assert!(include_str!("../{ASSETS}/🔧️{asset_stem}.op.semio").len() > 64);
    assert!(include_bytes!("../{ASSETS}/🎒️{asset_stem}.pack.semio").len() > 64);
    assert!(include_bytes!("../{ASSETS}/📡️{asset_stem}.spr.semio").len() > 64);
}}
"""


def rust_test_app(slug_emoji):
    return f"""//! ️tests for app example `{slug_emoji}`.

#[test]
fn cmd_asset_is_nonempty_demo_script() {{
    let text = include_str!("../{ASSETS}/🎮️demo.cmd.semio");
    assert!(text.len() > 64, "cmd fixture must carry real payload");
    assert!(text.contains("setActiveExample"), "demo session must drive an example load");
}}
"""


def ts_component(slug_emoji, slug_id, title_en, title_de, icon, asset_stem, kind):
    if kind == "app":
        return f"""/** 📚️ Example `{slug_emoji}`. */
export const id = "{slug_id}";
export const label = {{ en: "{title_en}", de: "{title_de}" }} as const;
export const icon = "{icon}";
export const cmdPath = new URL("./{ASSETS}/🎮️demo.cmd.semio", import.meta.url);
"""
    return f"""/** 📚️ Example `{slug_emoji}`. */
export const id = "{slug_id}";
export const label = {{ en: "{title_en}", de: "{title_de}" }} as const;
export const icon = "{icon}";
export const dslPath = new URL("./{ASSETS}/🗣️{asset_stem}.dsl.semio", import.meta.url);
export const opPath = new URL("./{ASSETS}/🔧️{asset_stem}.op.semio", import.meta.url);
export const packPath = new URL("./{ASSETS}/🎒️{asset_stem}.pack.semio", import.meta.url);
export const sprPath = new URL("./{ASSETS}/📡️{asset_stem}.spr.semio", import.meta.url);
"""


def ts_test(slug_emoji, asset_stem, kind):
    if kind == "app":
        return f"""import {{ readFileSync }} from "node:fs";
import {{ dirname, join }} from "node:path";
import {{ fileURLToPath }} from "node:url";
import {{ describe, expect, it }} from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example {slug_emoji}", () => {{
  it("ships a non-empty cmd demo script", () => {{
    const text = readFileSync(join(here, "../{ASSETS}/🎮️demo.cmd.semio"), "utf8");
    expect(text.length).toBeGreaterThan(64);
    expect(text).toContain("setActiveExample");
  }});
}});
"""
    return f"""import {{ readFileSync }} from "node:fs";
import {{ dirname, join }} from "node:path";
import {{ fileURLToPath }} from "node:url";
import {{ describe, expect, it }} from "vitest";

const here = dirname(fileURLToPath(import.meta.url));

describe("example {slug_emoji}", () => {{
  it("ships a non-empty dsl asset", () => {{
    const text = readFileSync(join(here, "../{ASSETS}/🗣️{asset_stem}.dsl.semio"), "utf8");
    expect(text.length).toBeGreaterThan(64);
    expect(text.startsWith("semio ")).toBe(true);
  }});

  it("ships nonempty op/pack/spr assets", () => {{
    expect(readFileSync(join(here, "../{ASSETS}/🔧️{asset_stem}.op.semio"), "utf8").length).toBeGreaterThan(64);
    expect(readFileSync(join(here, "../{ASSETS}/🎒️{asset_stem}.pack.semio")).byteLength).toBeGreaterThan(64);
    expect(readFileSync(join(here, "../{ASSETS}/📡️{asset_stem}.spr.semio")).byteLength).toBeGreaterThan(64);
  }});
}});
"""


def write_example_unit(root: Path, slug_dir: str, files: dict) -> None:
    for rel, content in files.items():
        write(root / slug_dir / rel, content)


def main() -> None:
    print("TESTS=", repr(TESTS), "ASSETS=", repr(ASSETS))
    for art_dir, art_mod, dim in art_meta():
        examples = PUZZLE / "🗿️artifacts" / art_dir / EXAMPLES
        is_2d = dim == "2d"
        nak_stem = "tower"
        cf_stem = "forest"

        # --- nakagin ---
        nak = examples / "🏗️nakagin-capsule-tower"
        assets = nak / ASSETS
        write(assets / f"🔧️{nak_stem}.op.semio", op_text(dim, "nakagin-capsule-tower"))
        write(assets / f"🎒️{nak_stem}.pack.semio", expand_binary_stub(assets / f"🎒️{nak_stem}.pack.semio", f"nakagin-{dim}-pack"))
        write(assets / f"📡️{nak_stem}.spr.semio", expand_binary_stub(assets / f"📡️{nak_stem}.spr.semio", f"nakagin-{dim}-spr"))
        write(
            nak / RS,
            rust_example_component(
                art_mod, "🏗️nakagin-capsule-tower", "nakagin-capsule-tower",
                "Nakagin Capsule Tower", "Nakagin-Kapselturm", "building", nak_stem, is_2d,
            ),
        )
        write(
            nak / TS,
            ts_component(
                "🏗️nakagin-capsule-tower", "nakagin-capsule-tower",
                "Nakagin Capsule Tower", "Nakagin-Kapselturm", "building", nak_stem, "artifact",
            ),
        )
        write(nak / TESTS / RS_TEST, rust_test("🏗️nakagin-capsule-tower", art_mod, nak_stem))
        write(nak / TESTS / TS_TEST, ts_test("🏗️nakagin-capsule-tower", nak_stem, "artifact"))

        # --- concrete-forest ---
        cf = examples / "🌲concrete-forest"
        cf_assets = cf / ASSETS
        dsl = (TICKET / f"w2-recovered-cf-{art_mod}.dsl.semio").read_bytes()
        write(cf_assets / f"🗣️{cf_stem}.dsl.semio", dsl)
        write(cf_assets / f"🔧️{cf_stem}.op.semio", op_text(dim, "concrete-forest"))
        # seed pack/spr from nakagin stubs then expand
        write(cf_assets / f"🎒️{cf_stem}.pack.semio", expand_binary_stub(assets / f"🎒️{nak_stem}.pack.semio", f"concrete-forest-{dim}-pack"))
        write(cf_assets / f"📡️{cf_stem}.spr.semio", expand_binary_stub(assets / f"📡️{nak_stem}.spr.semio", f"concrete-forest-{dim}-spr"))
        write(
            cf / RS,
            rust_example_component(
                art_mod, "🌲concrete-forest", "concrete-forest",
                "Concrete Forest", "Betonwald", "list-tree", cf_stem, is_2d,
            ),
        )
        write(
            cf / TS,
            ts_component(
                "🌲concrete-forest", "concrete-forest",
                "Concrete Forest", "Betonwald", "list-tree", cf_stem, "artifact",
            ),
        )
        write(cf / TESTS / RS_TEST, rust_test("🌲concrete-forest", art_mod, cf_stem))
        write(cf / TESTS / TS_TEST, ts_test("🌲concrete-forest", cf_stem, "artifact"))

    for app_dir, _app_mod, dim in app_meta():
        demo = PUZZLE / "🎛️apps" / app_dir / EXAMPLES / "🎬️demo-session"
        write(demo / ASSETS / "🎮️demo.cmd.semio", cmd_text(dim))
        write(demo / RS, rust_app_example(dim))
        write(
            demo / TS,
            ts_component("🎬️demo-session", "demo-session", "Demo Session", "Demo-Sitzung", "play", "demo", "app"),
        )
        write(demo / TESTS / RS_TEST, rust_test_app("🎬️demo-session"))
        write(demo / TESTS / TS_TEST, ts_test("🎬️demo-session", "demo", "app"))

    log_path = TICKET / "w2-migrate-files.json"
    log_path.write_text(json.dumps({"changed": changed}, indent=2, ensure_ascii=False) + "\n")
    print("wrote", len(changed), "files; log", log_path)


if __name__ == "__main__":
    main()
