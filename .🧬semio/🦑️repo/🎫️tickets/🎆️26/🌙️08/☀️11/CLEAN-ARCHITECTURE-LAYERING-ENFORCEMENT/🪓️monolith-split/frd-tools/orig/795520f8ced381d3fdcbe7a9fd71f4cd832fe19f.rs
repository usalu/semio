//! 🧾️ Every committed DSL carrier braces each record of a record list (`[ { k=v } ]`): the decoder has
//! no bare-record fallback, so `[` directly followed by `ident =` can never parse. Carriers are every DSL-language text: a
//! `*.dsl.semio` file or `include_str!` target whose first line is a `semio <kind>.<dsl|cmd|op> v<N>` header (raw-format
//! artifacts such as html or plain text keep their native bytes in `.dsl.semio` and carry no header). Headed carriers of a
//! handcrafted format grammar (e.g. STEP Part 21 under `stdio.ifc.dsl`) use their own lexical options, so the shared lexer
//! runs forgiving here: unknown characters become `Error` tokens and only the record-list token shape is judged.
use semio_framework_dsl::{lex, Limits, TokenKind};
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

fn repository_root() -> PathBuf {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !directory.join("nx.json").is_file() {
        assert!(directory.pop(), "repository root (nx.json) above {}", env!("CARGO_MANIFEST_DIR"));
    }
    directory
}

fn listed(root: &Path, pattern: &str) -> Vec<String> {
    let output = std::process::Command::new("git").args(["ls-files", "-co", "--exclude-standard", "-z", "--", pattern]).current_dir(root).output().expect("git lists repository files");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    output.stdout.split(|byte| *byte == 0).filter(|path| !path.is_empty()).map(|path| String::from_utf8(path.to_vec()).expect("utf-8 path")).filter(|path| !path.contains("/🎫️tickets/") && !path.contains("/target/")).collect()
}

fn normalized(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => { out.pop(); }
            other => out.push(other),
        }
    }
    out
}

fn headed(text: &str) -> bool {
    let first = text.lines().find(|line| !line.trim().is_empty()).unwrap_or("");
    let mut words = first.split_whitespace();
    let (Some("semio"), Some(kind), Some(version), None) = (words.next(), words.next(), words.next(), words.next()) else { return false };
    ["dsl", "cmd", "op"].iter().any(|suffix| kind.ends_with(&format!(".{suffix}"))) && version.strip_prefix('v').is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
}

fn included_carriers(root: &Path) -> BTreeSet<PathBuf> {
    let mut carriers = BTreeSet::new();
    for source in listed(root, "*.rs") {
        let Ok(text) = std::fs::read_to_string(root.join(&source)) else { continue };
        let mut rest = text.as_str();
        while let Some(at) = rest.find("include_str!(") {
            rest = &rest[at + "include_str!(".len()..];
            let Some(literal) = rest.trim_start().strip_prefix('"').and_then(|tail| tail.split_once('"')).map(|(path, _)| path) else { continue };
            if !literal.ends_with(".semio") { continue }
            let target = normalized(&Path::new(&source).parent().unwrap_or(Path::new("")).join(literal));
            if std::fs::read_to_string(root.join(&target)).is_ok_and(|content| headed(&content)) { carriers.insert(target); }
        }
    }
    carriers
}

#[test]
fn carrier_header_recognition_matches_the_declared_grammar() {
    assert!(headed("semio puzzle.puzzle2d.dsl v1\nnodes=[ ]"));
    assert!(headed("\n semio space.studio.dsl v12\n"));
    assert!(headed("semio draw.drawing.cmd v1"));
    assert!(!headed("schema=s.collection entries=[ ]"));
    assert!(!headed("semio puzzle.json v1"));
    assert!(!headed("semio puzzle.dsl vx"));
}

#[test]
fn committed_dsl_carriers_brace_every_record_list_item() {
    let root = repository_root();
    let mut carriers: BTreeSet<PathBuf> = listed(&root, "*.dsl.semio").into_iter().map(PathBuf::from).filter(|path| std::fs::read_to_string(root.join(path)).is_ok_and(|content| headed(&content))).collect();
    let committed = carriers.len();
    carriers.extend(included_carriers(&root));
    assert!(committed > 200, "expected the committed headed .dsl.semio corpus, found {committed}");
    assert!(carriers.len() > committed, "expected headed include_str! carriers beyond the .dsl.semio corpus");
    let limits = Limits { max_bytes: usize::MAX, max_tokens: usize::MAX, max_depth: usize::MAX, max_nodes: usize::MAX };
    let mut bare = Vec::new();
    for path in &carriers {
        let text = std::fs::read_to_string(root.join(path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let tokens: Vec<_> = lex(&text, &limits, true).unwrap_or_else(|error| panic!("{}: {error}", path.display())).into_iter().filter(|token| !token.kind.is_trivia()).collect();
        for window in tokens.windows(3) {
            if window[0].kind == TokenKind::LBracket && window[1].kind == TokenKind::Ident && window[2].kind == TokenKind::Equals {
                bare.push(format!("{}:{}", path.display(), text[..window[0].byte_range.0 as usize].matches('\n').count() + 1));
            }
        }
    }
    assert!(bare.is_empty(), "{} bare record lists in {} carriers:\n{}", bare.len(), carriers.len(), bare.join("\n"));
}
