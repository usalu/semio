#!/usr/bin/env python3
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SPECS = [
    ("✒️writer", "writer", "Writer", "WriterMutation", "apply_writer_mutation"),
    ("➗️mathematical", "mathematical", "Mathematical", "MathematicalMutation", "apply_mathematical_mutation"),
    ("🌊️flow", "flow", "Flow", "FlowMutation", "apply_flow_mutation"),
    ("🌿️vcs", "vcs", "Vcs", "VcsDemoMutation", "apply_vcs_demo_mutation"),
    ("🕸️dag", "dag", "Dag", "DagMutation", "apply_dag_mutation"),
]


def main() -> None:
    for plug, mod, prefix, mut, apply in SPECS:
        art = ROOT / "✏️s/🔌️plugins" / plug / "🗿️artifacts" / plug
        for rs in art.rglob("🦀️component.rs"):
            t = rs.read_text(encoding="utf-8")
            t2 = t.replace("::schema::diff::text::schema::", "::schema::diff::")
            if rs == art / "🏗️builder/🦀️component.rs":
                t2 = t2.replace(f"{apply}(&mut self.snapshot", f"crate::artifacts::{mod}::schema::mutations::{apply}(&mut self.snapshot")
            set_text = art / "🧬️schema/🧬️mutations/✍️set-text/🔺️diff/🦀️component.rs"
            if rs == set_text and "document:" in t2:
                t2 = t2.replace(
                    f"    pub fn into_{prefix.lower()}_diff(self) -> {prefix}Diff {{\n        {prefix}Diff {{ text: self.mutation.and_then(|m| match m {{ {mut}::SetText {{ text }} => Some(text), _ => None }}), document: None }}\n    }}",
                    f"""    pub fn into_{prefix.lower()}_diff(self) -> {prefix}Diff {{
        match self.mutation {{
            Some({mut}::SetText {{ text }}) => crate::artifacts::{mod}::schema::diff::text::diff_set_text(&text),
            _ => {prefix}Diff::default(),
        }}
    }}""",
                )
            if t != t2:
                rs.write_text(t2, encoding="utf-8")
                print(plug, rs.relative_to(art))


if __name__ == "__main__":
    main()
