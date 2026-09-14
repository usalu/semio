//! 🏗️ Emits the five committed fixture files of `🔨️modules/🚚️move/🧫️fixtures/`. The INPUTS below are
//! hand written; the recorded expectations are produced by running the crate's own planners over
//! them, so a fixture can never disagree with the implementation by transcription error. Rerun with
//! `cargo run --manifest-path <ticket>/🗑️generated/move/fixture-gen/Cargo.toml -- <fixtures dir>`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use semio_framework_repo_move::{
    apply_rename_casings, execute, plan_extract, plan_file_move, plan_folder_move, plan_integrate, plan_rename, plan_section_move, Plan, Workspace,
};

//#region 🔖️Json

fn quote(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

enum J {
    S(String),
    N(i64),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

impl J {
    fn render(&self, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        let inner = "  ".repeat(indent + 1);
        match self {
            J::S(value) => quote(value),
            J::N(value) => value.to_string(),
            J::A(items) if items.is_empty() => "[]".to_string(),
            J::A(items) => {
                let body: Vec<String> = items.iter().map(|i| format!("{inner}{}", i.render(indent + 1))).collect();
                format!("[\n{}\n{pad}]", body.join(",\n"))
            }
            J::O(entries) if entries.is_empty() => "{}".to_string(),
            J::O(entries) => {
                let body: Vec<String> = entries.iter().map(|(k, v)| format!("{inner}{}: {}", quote(k), v.render(indent + 1))).collect();
                format!("{{\n{}\n{pad}}}", body.join(",\n"))
            }
        }
    }
}

fn s(value: &str) -> J {
    J::S(value.to_string())
}

//#endregion 🔖️Json

//#region 🔖️Trees

fn tree_json(workspace: &Workspace) -> J {
    let directories: Vec<J> = workspace
        .iter()
        .filter(|(_, entry)| matches!(entry, semio_framework_repo_move::Entry::Directory))
        .map(|(path, _)| s(path))
        .collect();
    let files: Vec<(String, J)> = workspace.files().into_iter().map(|(path, content)| (path, J::S(content))).collect();
    J::O(vec![("directories".to_string(), J::A(directories)), ("files".to_string(), J::O(files))])
}

fn applied(workspace: &Workspace, plan: &Plan) -> Workspace {
    let mut copy = workspace.clone();
    execute(plan, &mut copy).expect("plan applies");
    copy
}

fn stats_json(plan: &Plan) -> J {
    J::O(plan.stats.iter().map(|(key, value)| (key.clone(), J::N(*value))).collect())
}

fn messages_json(plan: &Plan) -> J {
    J::A(plan.lines().into_iter().map(J::S).collect())
}

//#endregion 🔖️Trees

//#region 🔖️Rename

fn rename_fixture() -> J {
    let token_inputs: Vec<(&str, &str, &str, &str)> = vec![
        ("plain-lower-token", "model", "representation", "the model of a model is a MODEL and a Model"),
        ("screaming-and-title", "kit", "collection", "KIT Kit kit kITs"),
        ("kebab-token", "design-piece", "layout-part", "DESIGN-PIECE Design-piece design-piece"),
        ("snake-token", "type_id", "kind_id", "TYPE_ID Type_id type_id"),
        ("camel-token", "typeId", "kindId", "TYPEID TypeId typeid typeId"),
        ("pascal-token", "TypeId", "KindId", "TYPEID TypeId typeid"),
        ("token-inside-a-path", "model", "shape", "🔨️modules/📐️model/📦️packages/MODEL.ts"),
        ("no-occurrence", "absent", "present", "nothing to see here"),
    ];
    let vectors: Vec<J> = token_inputs
        .iter()
        .map(|(name, old, new, content)| {
            J::O(vec![
                ("name".to_string(), s(name)),
                ("old".to_string(), s(old)),
                ("new".to_string(), s(new)),
                ("content".to_string(), s(content)),
                ("expected".to_string(), J::S(apply_rename_casings(content, old, new))),
            ])
        })
        .collect();

    let mut trees: Vec<J> = Vec::new();
    for (name, old, new, scope, build) in rename_trees() {
        let before = build();
        let plan = plan_rename(&before, old, new, scope).expect("rename plans");
        let after = applied(&before, &plan);
        trees.push(J::O(vec![
            ("name".to_string(), s(name)),
            ("old".to_string(), s(old)),
            ("new".to_string(), s(new)),
            ("scope".to_string(), s(scope)),
            ("before".to_string(), tree_json(&before)),
            ("after".to_string(), tree_json(&after)),
            ("stats".to_string(), stats_json(&plan)),
            ("messages".to_string(), messages_json(&plan)),
        ]));
    }

    let errors: Vec<J> = vec![
        rename_error("an-empty-token-is-refused", "", "target", ""),
        rename_error("an-identical-token-is-refused", "Model", "model", ""),
        rename_error("an-unknown-scope-is-refused", "model", "shape", "nowhere"),
    ];

    J::O(vec![
        ("schema".to_string(), s("https://semio.tech/schema/repo/move/1#/$defs/RenameVectorFile")),
        ("tokenVectors".to_string(), J::A(vectors)),
        ("trees".to_string(), J::A(trees)),
        ("errors".to_string(), J::A(errors)),
    ])
}

fn rename_error(name: &str, old: &str, new: &str, scope: &str) -> J {
    let workspace = rename_workspace();
    let message = plan_rename(&workspace, old, new, scope).err().expect("refused").0;
    J::O(vec![
        ("name".to_string(), s(name)),
        ("old".to_string(), s(old)),
        ("new".to_string(), s(new)),
        ("scope".to_string(), s(scope)),
        ("before".to_string(), tree_json(&workspace)),
        ("expectedError".to_string(), J::S(message)),
    ])
}

fn rename_workspace() -> Workspace {
    Workspace::new()
        .with_file("model/model.ts", "export const model = \"MODEL\";\n// Model of a model\n")
        .with_file("model/nested/Model.md", "# Model\n\nA MODEL describes a model.\n")
        .with_file("docs/readme.md", "The model is not the Model.\n")
        .with_file("node_modules/model/index.js", "module.exports = \"model\";\n")
        .with_file(".git/config", "[core]\n\tmodel = true\n")
        .with_directory("empty-model")
}

type RenameTree = (&'static str, &'static str, &'static str, &'static str, fn() -> Workspace);

fn rename_trees() -> Vec<RenameTree> {
    vec![
        ("whole-tree", "model", "shape", "", rename_workspace as fn() -> Workspace),
        ("scoped-to-one-directory", "model", "shape", "model", rename_workspace as fn() -> Workspace),
        ("only-a-filename-changes", "readme", "overview", "docs", rename_workspace as fn() -> Workspace),
    ]
}

//#endregion 🔖️Rename

//#region 🔖️Sections

const TYPESCRIPT_FILE: &str = "//#region 🧲️Header\n\n// 2026 Ueli Saluz\n\n//#endregion 🧲️Header\n\nimport { join } from \"node:path\";\n\n// #region 🔖️Alpha\nexport const alpha = join(\"a\");\n// #endregion 🔖️Alpha\n\n// #region 🔖️Beta\nexport const beta = 2;\n// #endregion 🔖️Beta\n";

const GO_FILE: &str = "// #region 🔖Header\n\n// 2026 Ueli Saluz\n\n// #endregion 🔖Header\n\npackage sample\n\nimport (\n\t\"fmt\"\n\t\"strings\"\n)\n\n// #region 🔖Alpha\nfunc Alpha() string { return fmt.Sprint(strings.ToUpper(\"a\")) }\n// #endregion 🔖Alpha\n\n// #region 🔖Beta\nfunc Beta() int { return 2 }\n// #endregion 🔖Beta\n";

const MARKDOWN_FILE: &str = "# Title\n\n## Alpha\n\nSome alpha text.\n\n## Beta\n\nSome beta text.\n";

const PYTHON_FILE: &str = "# #region 🔖️Header\n\n# 2026 Ueli Saluz\n\n# #endregion 🔖️Header\n\nimport os\nfrom pathlib import Path\n\n# #region 🔖️Alpha\ndef alpha():\n    return os.sep + str(Path(\".\"))\n# #endregion 🔖️Alpha\n";

fn section_move_fixture() -> J {
    let inputs: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("typescript-markers", "sample.ts", TYPESCRIPT_FILE, "Alpha", "Gamma"),
        ("go-markers", "sample.go", GO_FILE, "Beta", "Delta"),
        ("markdown-headings", "sample.md", MARKDOWN_FILE, "Alpha", "Gamma"),
        ("python-markers", "sample.py", PYTHON_FILE, "Alpha", "Gamma"),
        ("nested-path-takes-the-leaf", "sample.ts", TYPESCRIPT_FILE, "Outer#Alpha", "Outer#Gamma"),
        ("an-absent-section-leaves-the-file-alone", "sample.ts", TYPESCRIPT_FILE, "Missing", "Gamma"),
    ];
    let cases: Vec<J> = inputs
        .iter()
        .map(|(name, file, content, old, new)| {
            let workspace = Workspace::new().with_file(file, content);
            let plan = plan_section_move(&workspace, file, old, new).expect("section move plans");
            let after = applied(&workspace, &plan);
            J::O(vec![
                ("name".to_string(), s(name)),
                ("file".to_string(), s(file)),
                ("content".to_string(), s(content)),
                ("oldPath".to_string(), s(old)),
                ("newPath".to_string(), s(new)),
                ("expected".to_string(), J::S(after.file(file).unwrap_or_default().to_string())),
                ("messages".to_string(), messages_json(&plan)),
            ])
        })
        .collect();
    let missing = Workspace::new().with_file("sample.ts", TYPESCRIPT_FILE);
    let error = plan_section_move(&missing, "absent.ts", "Alpha", "Gamma").err().expect("refused").0;
    J::O(vec![
        ("schema".to_string(), s("https://semio.tech/schema/repo/move/1#/$defs/SectionMoveFile")),
        ("cases".to_string(), J::A(cases)),
        (
            "errors".to_string(),
            J::A(vec![J::O(vec![
                ("name".to_string(), s("a-missing-file-is-refused")),
                ("file".to_string(), s("absent.ts")),
                ("oldPath".to_string(), s("Alpha")),
                ("newPath".to_string(), s("Gamma")),
                ("expectedError".to_string(), J::S(error)),
            ])]),
        ),
    ])
}

fn section_extract_fixture() -> J {
    let inputs: Vec<(&str, &str, &str, &str, &str)> = vec![
        ("typescript-section-takes-the-imports", "sample.ts", TYPESCRIPT_FILE, "Alpha", "alpha.ts"),
        ("go-section-takes-the-package-and-imports", "sample.go", GO_FILE, "Beta", "beta.go"),
        ("python-section-takes-the-imports", "sample.py", PYTHON_FILE, "Alpha", "alpha.py"),
        ("markdown-section-has-no-imports", "sample.md", MARKDOWN_FILE, "Alpha", "alpha.md"),
    ];
    let cases: Vec<J> = inputs
        .iter()
        .map(|(name, file, content, section, target)| {
            let workspace = Workspace::new().with_file(file, content);
            let plan = plan_extract(&workspace, file, section, target).expect("extract plans");
            let after = applied(&workspace, &plan);
            J::O(vec![
                ("name".to_string(), s(name)),
                ("sourceFile".to_string(), s(file)),
                ("content".to_string(), s(content)),
                ("section".to_string(), s(section)),
                ("targetFile".to_string(), s(target)),
                ("expectedTarget".to_string(), J::S(after.file(target).unwrap_or_default().to_string())),
                ("expectedSource".to_string(), J::S(after.file(file).unwrap_or_default().to_string())),
                ("messages".to_string(), messages_json(&plan)),
            ])
        })
        .collect();
    let workspace = Workspace::new().with_file("sample.ts", TYPESCRIPT_FILE).with_file("plain.txt", "no sections here\n");
    let errors: Vec<J> = vec![
        extract_error(&workspace, "a-missing-source-is-refused", "absent.ts", "Alpha", "out.ts"),
        extract_error(&workspace, "a-missing-section-is-refused", "sample.ts", "Missing", "out.ts"),
        extract_error(&workspace, "a-sectionless-language-is-refused", "plain.txt", "Alpha", "out.txt"),
    ];
    J::O(vec![
        ("schema".to_string(), s("https://semio.tech/schema/repo/move/1#/$defs/SectionExtractFile")),
        ("cases".to_string(), J::A(cases)),
        ("errors".to_string(), J::A(errors)),
    ])
}

fn extract_error(workspace: &Workspace, name: &str, file: &str, section: &str, target: &str) -> J {
    let message = plan_extract(workspace, file, section, target).err().expect("refused").0;
    J::O(vec![
        ("name".to_string(), s(name)),
        ("sourceFile".to_string(), s(file)),
        ("section".to_string(), s(section)),
        ("targetFile".to_string(), s(target)),
        ("expectedError".to_string(), J::S(message)),
    ])
}

const TYPESCRIPT_SOURCE_FILE: &str = "//#region 🧲️Header\n\n// 2026 Someone Else\n\n//#endregion 🧲️Header\n\nimport { basename } from \"node:path\";\n\nexport const gamma = basename(\"g\");\n";

const GO_SOURCE_FILE: &str = "// #region 🔖Header\n\n// 2026 Someone Else\n\n// #endregion 🔖Header\n\npackage other\n\nimport (\n\t\"errors\"\n\t\"fmt\"\n)\n\nfunc Gamma() error { return fmt.Errorf(\"%w\", errors.New(\"g\")) }\n";

fn file_integrate_fixture() -> J {
    let inputs: Vec<(&str, &str, &str, &str, &str, &str, &str)> = vec![
        ("typescript-appended-at-the-end", "gamma.ts", TYPESCRIPT_SOURCE_FILE, "sample.ts", TYPESCRIPT_FILE, "Gamma", ""),
        ("typescript-inside-a-parent-section", "gamma.ts", TYPESCRIPT_SOURCE_FILE, "sample.ts", TYPESCRIPT_FILE, "Gamma", "Beta"),
        ("go-keeps-the-target-package", "gamma.go", GO_SOURCE_FILE, "sample.go", GO_FILE, "Gamma", ""),
    ];
    let cases: Vec<J> = inputs
        .iter()
        .map(|(name, source, source_content, target, target_content, section, parent)| {
            let workspace = Workspace::new().with_file(source, source_content).with_file(target, target_content);
            let plan = plan_integrate(&workspace, source, section, target, parent).expect("integrate plans");
            let after = applied(&workspace, &plan);
            J::O(vec![
                ("name".to_string(), s(name)),
                ("sourceFile".to_string(), s(source)),
                ("sourceContent".to_string(), s(source_content)),
                ("targetFile".to_string(), s(target)),
                ("targetContent".to_string(), s(target_content)),
                ("targetSection".to_string(), s(section)),
                ("parentSection".to_string(), s(parent)),
                ("expected".to_string(), J::S(after.file(target).unwrap_or_default().to_string())),
                ("messages".to_string(), messages_json(&plan)),
            ])
        })
        .collect();
    let workspace = Workspace::new()
        .with_file("gamma.ts", TYPESCRIPT_SOURCE_FILE)
        .with_file("sample.ts", TYPESCRIPT_FILE)
        .with_file("plain.txt", "no sections here\n");
    let errors: Vec<J> = vec![
        integrate_error(&workspace, "a-missing-source-is-refused", "absent.ts", "Gamma", "sample.ts", ""),
        integrate_error(&workspace, "a-missing-target-is-refused", "gamma.ts", "Gamma", "absent.ts", ""),
        integrate_error(&workspace, "a-sectionless-target-is-refused", "gamma.ts", "Gamma", "plain.txt", ""),
        integrate_error(&workspace, "an-unknown-parent-section-is-refused", "gamma.ts", "Gamma", "sample.ts", "Missing"),
    ];
    J::O(vec![
        ("schema".to_string(), s("https://semio.tech/schema/repo/move/1#/$defs/FileIntegrateFile")),
        ("cases".to_string(), J::A(cases)),
        ("errors".to_string(), J::A(errors)),
    ])
}

fn integrate_error(workspace: &Workspace, name: &str, source: &str, section: &str, target: &str, parent: &str) -> J {
    let message = plan_integrate(workspace, source, section, target, parent).err().expect("refused").0;
    J::O(vec![
        ("name".to_string(), s(name)),
        ("sourceFile".to_string(), s(source)),
        ("targetFile".to_string(), s(target)),
        ("targetSection".to_string(), s(section)),
        ("parentSection".to_string(), s(parent)),
        ("expectedError".to_string(), J::S(message)),
    ])
}

//#endregion 🔖️Sections

//#region 🔖️Paths

fn move_workspace() -> Workspace {
    Workspace::new()
        .with_file("AGENTS.md", "# Agents\n\n## 📁️src/old/\n\nThe old home.\n\n### 📄️src/old/main.ts\n\nOne file.\n")
        .with_file("src/old/main.ts", "export const main = 1;\n")
        .with_file("src/old/deep/util.ts", "export const util = 2;\n")
        .with_file("src/keep.ts", "export const keep = 3;\n")
        .with_directory("src/target")
}

fn file_folder_move_fixture() -> J {
    let inputs: Vec<(&str, &str, &str, &str)> = vec![
        ("a-folder-carries-its-subtree", "folder", "src/old", "src/new"),
        ("a-folder-into-an-existing-parent", "folder", "src/old", "src/target/moved"),
        ("a-file-moves-alone", "file", "src/old/main.ts", "src/target/main.ts"),
        ("a-file-with-no-docs-entry", "file", "src/keep.ts", "src/kept.ts"),
    ];
    let cases: Vec<J> = inputs
        .iter()
        .map(|(name, kind, source, target)| {
            let before = move_workspace();
            let plan = if *kind == "folder" {
                plan_folder_move(&before, source, target).expect("folder move plans")
            } else {
                plan_file_move(&before, source, target).expect("file move plans")
            };
            let after = applied(&before, &plan);
            J::O(vec![
                ("name".to_string(), s(name)),
                ("kind".to_string(), s(kind)),
                ("source".to_string(), s(source)),
                ("target".to_string(), s(target)),
                ("before".to_string(), tree_json(&before)),
                ("after".to_string(), tree_json(&after)),
                ("messages".to_string(), messages_json(&plan)),
            ])
        })
        .collect();
    let before = move_workspace();
    let errors: Vec<J> = vec![
        move_error(&before, "a-missing-source-folder-is-refused", "folder", "src/absent", "src/new"),
        move_error(&before, "an-occupied-folder-target-is-refused", "folder", "src/old", "src/target"),
        move_error(&before, "a-missing-source-file-is-refused", "file", "src/absent.ts", "src/new.ts"),
        move_error(&before, "an-occupied-file-target-is-refused", "file", "src/keep.ts", "src/old/main.ts"),
    ];
    J::O(vec![
        ("schema".to_string(), s("https://semio.tech/schema/repo/move/1#/$defs/FileFolderMoveFile")),
        ("cases".to_string(), J::A(cases)),
        ("errors".to_string(), J::A(errors)),
    ])
}

fn move_error(workspace: &Workspace, name: &str, kind: &str, source: &str, target: &str) -> J {
    let message = if kind == "folder" {
        plan_folder_move(workspace, source, target).err().expect("refused").0
    } else {
        plan_file_move(workspace, source, target).err().expect("refused").0
    };
    J::O(vec![
        ("name".to_string(), s(name)),
        ("kind".to_string(), s(kind)),
        ("source".to_string(), s(source)),
        ("target".to_string(), s(target)),
        ("expectedError".to_string(), J::S(message)),
    ])
}

//#endregion 🔖️Paths

//#region 🔖️Main

pub fn main() {
    let target = PathBuf::from(std::env::args().nth(1).expect("fixtures directory argument"));
    let files: BTreeMap<&str, J> = BTreeMap::from([
        ("🔤️rename-vectors.json", rename_fixture()),
        ("📑️section-move-trees.json", section_move_fixture()),
        ("🧲️section-extract-trees.json", section_extract_fixture()),
        ("📥️file-integrate-trees.json", file_integrate_fixture()),
        ("🚚️file-folder-move-trees.json", file_folder_move_fixture()),
    ]);
    for (name, document) in files {
        let path = target.join(name);
        std::fs::write(&path, format!("{}\n", document.render(0))).expect("fixture written");
        println!("[fixtures] wrote {}", path.display());
    }
}

//#endregion 🔖️Main
