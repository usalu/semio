#!/usr/bin/env python3
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGS = ["✒️writer", "➗️mathematical", "🌊️flow", "🌿️vcs", "🕸️dag"]
APPLY = {
    "writer": "apply_writer_mutation",
    "mathematical": "apply_mathematical_mutation",
    "flow": "apply_flow_mutation",
    "vcs": "apply_vcs_demo_mutation",
    "dag": "apply_dag_mutation",
}


def main() -> None:
    for plug in PLUGS:
        mod = plug.encode("utf-8").decode("unicode_escape") if False else {
            "✒️writer": "writer",
            "➗️mathematical": "mathematical",
            "🌊️flow": "flow",
            "🌿️vcs": "vcs",
            "🕸️dag": "dag",
        }[plug]
        art = ROOT / "✏️s/🔌️plugins" / plug / "🗿️artifacts" / plug
        for rs in art.rglob("🦀️component.rs"):
            t = rs.read_text(encoding="utf-8")
            t2 = t.replace(f"::schema::diff::text::schema::", "::schema::diff::")
            t2 = t2.replace(f"::schema::diff::text::WriterDiff", "::schema::diff::WriterDiff")
            if rs == art / "🏗️builder/🦀️component.rs":
                t2 = t2.replace(
                    f"{APPLY[mod]}(&mut self.snapshot",
                    f"crate::artifacts::{mod}::schema::mutations::{APPLY[mod]}(&mut self.snapshot",
                )
            if "into_writer_diff" in t2 and "document:" in t2:
                t2 = t2.replace(
                    "WriterDiff { text: self.mutation.and_then(|m| match m { WriterMutation::SetText { text } => Some(text), _ => None }), document: None }",
                    "crate::artifacts::writer::schema::diff::text::diff_set_text(self.mutation.as_ref().and_then(|m| match m { WriterMutation::SetText { text } => Some(text.as_str()), _ => None }).unwrap_or(\"\"))",
                )
            if t != t2:
                rs.write_text(t2, encoding="utf-8")
                print(rs.relative_to(ROOT))


if __name__ == "__main__":
    main()
