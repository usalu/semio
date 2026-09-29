#!/usr/bin/env python3
"""🧪️ U6 set A4 — the last stdio lib-test drift (test-only, rule 22): window-owned verbs and the bcf fixture writer.

* semio brep/mesh + wav: the app roster is "every action … that no window kind claims as its own" (`AppDefinition::actions`,
  `🛂️manifest`); `set-vertex` and wav's audio verbs are owned by their edit windows, so the laws look the verb up where the
  definition declares it — the app roster and every window kind's actions.
* bcf: `fixture_honesty_law` had no writer (svg/xlsx/docx have `zzz_write_…`), so a codec change could only be followed by a hand
  edit; the ignored writer `zzz_write_demo_fixtures` regenerates both demo assets from `demo_bcf_snapshot()` (run in the lane).
* semio brep structural-copy cancellation: the retirement ladder pages a partially copied snapshot per owned value now (measured
  22 541 one-item close turns for 4 096 copied steps, converging); the bound is stated per copied item (8 × 4 096) instead of the old
  flat 20 000.

Usage: u6-test-drift-4.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-4")
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
SEMIO = ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
LOOKUP_OLD = 'definition.actions.iter().find(|action| action.id == "set-vertex").expect("set-vertex action");\n'
LOOKUP_NEW = 'definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "set-vertex").expect("set-vertex action");\n'
WRITER_OLD = """        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_bcf_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
"""
WRITER_NEW = """        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_bcf_snapshot()) drifted from the shipped .pack.semio fixture");
    }

    /// 🖊️ The ONLY way those two fixtures are ever refreshed: `print_dsl`/`encode_pack` of the demo itself, never a hand edit
    /// (`fixture_honesty_law` above is what that honesty means). Run it deliberately after a codec change —
    /// `cargo test -p semio-s-artifact-stdio-bcf --lib -- --ignored zzz_write_demo_fixtures` — then re-run the law.
    #[semio_framework_async_macros::async_test]
    #[ignore]
    async fn zzz_write_demo_fixtures() {
        let demo = demo_bcf_snapshot();
        let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/📚️examples/🎬️demo/🖼️assets");
        std::fs::write(assets.join("🗣️.dsl.semio"), store::ArtifactDsl::print_dsl(&demo)).expect("write 🗣️.dsl.semio");
        std::fs::write(assets.join("🎒️.pack.semio"), store::ArtifactPack::encode_pack(&demo)).expect("write 🎒️.pack.semio");
    }
}
//#endregion 🔖️ConformanceLaws
"""
SETS = {
    SEMIO + "🧊️brep/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [(LOOKUP_OLD, LOOKUP_NEW, 1)],
    SEMIO + "🔺️mesh/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [(LOOKUP_OLD, LOOKUP_NEW, 1)],
    ART + "🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        (
            '        let action = definition.actions.iter().find(|action| action.id == *action_id).expect("natural audio action");\n',
            '        let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == *action_id).expect("natural audio action");\n',
            1,
        )
    ],
    ART + "💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🧪️tests/🔬️unit/🦀️.rs": [(WRITER_OLD, WRITER_NEW, 1)],
    SEMIO + "🧊️brep/✏️editor/📬️preparation/🦀️.rs": [
        (
            "            assert!(close_turns < 20_000);\n",
            "            assert!(close_turns < 8 * 4_096, \"a cancelled copy of 4 096 one-item steps retires within 8 retirement units per copied item\");\n",
            1,
        )
    ],
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/🗿️artifacts/', 1)[-1][:60]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
