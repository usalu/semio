use std::{fs, path::{Component, Path, PathBuf}};
//#region 🔖️MutationSourceAuthority
#[derive(Debug)]
struct MutationSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    owner: String,
    source_path: PathBuf,
    descriptor_path: PathBuf,
    taxonomy_path: PathBuf,
}

fn mutation_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationSourceAuthority, String> {
    let source_path = mutation_authority_normalize(source, compiler_cwd)?;
    let workspace_root = mutation_authority_workspace_root(&source_path)?;
    mutation_authority_no_follow(&workspace_root, &source_path, false)?;
    let project_path = workspace_root.join("📋️project.json");
    mutation_authority_no_follow(&workspace_root, &project_path, false)?;
    let project: serde_json::Value = serde_json::from_slice(&fs::read(&project_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    let locator = project.pointer("/metadata/semio/taxonomy").and_then(serde_json::Value::as_str).ok_or_else(|| "missing metadata.semio.taxonomy".to_string())?;
    let taxonomy_path = mutation_authority_locator(&workspace_root, locator)?;
    mutation_authority_no_follow(&workspace_root, &taxonomy_path, false)?;
    let taxonomy: serde_json::Value = serde_json::from_slice(&fs::read(&taxonomy_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    let source_filename = mutation_authority_filename(&taxonomy, taxonomy.get("mutationComponentFileKindId").and_then(serde_json::Value::as_str).ok_or_else(|| "missing mutationComponentFileKindId".to_string())?)?;
    let descriptor_filename = mutation_authority_filename(&taxonomy, taxonomy.get("mutationDescriptorFileKindId").and_then(serde_json::Value::as_str).ok_or_else(|| "missing mutationDescriptorFileKindId".to_string())?)?;
    if source_path.file_name().and_then(|name| name.to_str()) != Some(source_filename.as_str()) { return Err("source is not the taxonomy canonical mutation primary".to_string()); }
    let mutation_collection = mutation_authority_collection(&taxonomy)?;
    let owner_path = source_path.parent().ok_or_else(|| "source has no owner directory".to_string())?;
    let mutation_root = owner_path.parent().ok_or_else(|| "source owner has no collection parent".to_string())?;
    if mutation_root.file_name().and_then(|name| name.to_str()) != Some(mutation_collection.as_str()) { return Err("source owner is not a direct mutation collection leaf".to_string()); }
    let owner = mutation_authority_relative(&workspace_root, owner_path)?;
    let descriptor_path = owner_path.join(descriptor_filename);
    mutation_authority_no_follow(&workspace_root, &descriptor_path, false)?;
    let descriptor: serde_json::Value = serde_json::from_slice(&fs::read(&descriptor_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    if descriptor.as_object().is_none() || descriptor.get("owner").and_then(serde_json::Value::as_str) != Some(owner.as_str()) { return Err("descriptor owner does not exactly match source owner".to_string()); }
    Ok(MutationSourceAuthority { workspace_root, mutation_root: mutation_root.to_path_buf(), owner, source_path, descriptor_path, taxonomy_path })
}

fn mutation_authority_normalize(source: &Path, compiler_cwd: &Path) -> Result<PathBuf, String> {
    if !compiler_cwd.is_absolute() { return Err("compiler cwd is not absolute".to_string()); }
    mutation_authority_raw_lexical(compiler_cwd)?;
    let input = if source.is_absolute() { source.to_path_buf() } else { compiler_cwd.join(source) };
    mutation_authority_raw_no_follow(&input)?;
    let mut normalized = PathBuf::new();
    for component in input.components() {
        if let Component::Normal(segment) = component {
            if segment.to_str().map(|value| value.eq_ignore_ascii_case("compose")).unwrap_or(false) { return Err("opaque compose path rejected before I/O".to_string()); }
        }
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {},
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => { if !normalized.pop() { return Err("source escapes filesystem root".to_string()); } },
        }
    }
    if !normalized.is_absolute() { return Err("source is not absolute after compiler cwd resolution".to_string()); }
    Ok(normalized)
}

fn mutation_authority_raw_no_follow(path: &Path) -> Result<(), String> {
    if !path.is_absolute() { return Err("raw source path is not absolute".to_string()); }
    mutation_authority_raw_lexical(path)?;
    let mut current = PathBuf::new();
    let components: Vec<Component<'_>> = path.components().collect();
    for (index, component) in components.iter().enumerate() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
            Component::RootDir => { current.push(component.as_os_str()); let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?; if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() { return Err("raw source root is not a regular directory".to_string()); } },
            Component::CurDir => {},
            Component::Normal(segment) => {
                current.push(segment);
                let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
                if metadata.file_type().is_symlink() { return Err("raw source symlink component rejected before normalization".to_string()); }
                if components[index + 1..].iter().any(|next| matches!(next, Component::Normal(_) | Component::ParentDir)) && !metadata.file_type().is_dir() { return Err("raw source intermediate component is not a directory".to_string()); }
                if !components[index + 1..].iter().any(|next| matches!(next, Component::Normal(_) | Component::ParentDir)) && !metadata.file_type().is_file() { return Err("raw source terminal component is not a regular file".to_string()); }
            },
            Component::ParentDir => { if !current.pop() { return Err("source escapes filesystem root".to_string()); } },
        }
    }
    Ok(())
}

fn mutation_authority_raw_lexical(path: &Path) -> Result<(), String> {
    for component in path.components() {
        if let Component::Normal(segment) = component {
            let segment = segment.to_str().ok_or_else(|| "raw source path component is not UTF-8".to_string())?;
            if segment.contains('\0') || segment.contains('\\') { return Err("raw source path component is not a portable owner segment".to_string()); }
            if segment.eq_ignore_ascii_case("compose") { return Err("opaque compose path rejected before I/O".to_string()); }
        }
    }
    Ok(())
}

fn mutation_authority_workspace_root(source: &Path) -> Result<PathBuf, String> {
    for ancestor in source.parent().into_iter().flat_map(Path::ancestors) {
        let nx = ancestor.join("nx.json");
        let project = ancestor.join("📋️project.json");
        match mutation_authority_node(&nx) {
            "missing" => continue,
            "symlink" => return Err("workspace nx.json marker is a symlink".to_string()),
            "file" => {},
            _ => return Err("workspace nx.json marker is not a regular file".to_string()),
        }
        match mutation_authority_node(&project) {
            "file" => return Ok(ancestor.to_path_buf()),
            "missing" => return Err("workspace nx.json marker lacks paired 📋️project.json".to_string()),
            "symlink" => return Err("workspace 📋️project.json marker is a symlink".to_string()),
            _ => return Err("workspace 📋️project.json marker is not a regular file".to_string()),
        }
    }
    Err("source has no exact nx.json and 📋️project.json workspace pair".to_string())
}

fn mutation_authority_node(path: &Path) -> &'static str {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => "symlink",
        Ok(metadata) if metadata.file_type().is_file() => "file",
        Ok(_) => "other",
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => "missing",
        Err(_) => "other",
    }
}

fn mutation_authority_no_follow(root: &Path, target: &Path, directory: bool) -> Result<(), String> {
    let relative = target.strip_prefix(root).map_err(|_| "path escapes workspace root".to_string())?;
    let mut current = root.to_path_buf();
    let root_metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
    if root_metadata.file_type().is_symlink() || !root_metadata.file_type().is_dir() { return Err("workspace root is not a regular directory".to_string()); }
    for component in relative.components() {
        let Component::Normal(segment) = component else { return Err("path is not lexically normalized".to_string()); };
        if segment.to_str().map(|value| value.eq_ignore_ascii_case("compose")).unwrap_or(false) { return Err("opaque compose path rejected before I/O".to_string()); }
        current.push(segment);
        let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() { return Err("symlink path component rejected".to_string()); }
    }
    let metadata = fs::symlink_metadata(target).map_err(|error| error.to_string())?;
    if directory && !metadata.file_type().is_dir() { return Err("expected regular directory".to_string()); }
    if !directory && !metadata.file_type().is_file() { return Err("expected regular file".to_string()); }
    Ok(())
}

fn mutation_authority_locator(root: &Path, locator: &str) -> Result<PathBuf, String> {
    if locator.is_empty() || locator.contains('\0') || locator.contains('\\') || locator.starts_with('/') || locator.as_bytes().get(1) == Some(&b':') { return Err("taxonomy locator is not normalized repository-relative path".to_string()); }
    let mut target = root.to_path_buf();
    for segment in locator.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("taxonomy locator has rejected path component".to_string()); }
        target.push(segment);
    }
    Ok(target)
}

fn mutation_authority_filename(taxonomy: &serde_json::Value, file_kind_id: &str) -> Result<String, String> {
    let file_kind = taxonomy.get("fileKinds").and_then(serde_json::Value::as_object).and_then(|kinds| kinds.get(file_kind_id)).and_then(serde_json::Value::as_object).ok_or_else(|| "taxonomy file kind is missing".to_string())?;
    let emoji = file_kind.get("emoji").and_then(serde_json::Value::as_str).filter(|value| !value.is_empty()).ok_or_else(|| "taxonomy file kind emoji is missing".to_string())?;
    let extension = file_kind.get("extensionChains").and_then(serde_json::Value::as_array).and_then(|values| values.first()).and_then(serde_json::Value::as_str).filter(|value| value.starts_with('.')).ok_or_else(|| "taxonomy file kind canonical extension is missing".to_string())?;
    Ok(format!("{emoji}{extension}"))
}

fn mutation_authority_collection(taxonomy: &serde_json::Value) -> Result<String, String> {
    let collections = taxonomy.get("semanticCollections").and_then(serde_json::Value::as_object).ok_or_else(|| "taxonomy semantic collections are missing".to_string())?;
    let mutations: Vec<&String> = collections.iter().filter_map(|(name, definition)| (definition.get("kind").and_then(serde_json::Value::as_str) == Some("mutation")).then_some(name)).collect();
    if mutations.len() != 1 || mutations[0].contains('/') { return Err("taxonomy mutation collection is ambiguous".to_string()); }
    Ok(mutations[0].to_string())
}

fn mutation_authority_relative(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|_| "owner escapes workspace root".to_string())?;
    let mut segments = Vec::new();
    for component in relative.components() {
        let Component::Normal(segment) = component else { return Err("owner is not normalized".to_string()); };
        let segment = segment.to_str().ok_or_else(|| "owner is not UTF-8".to_string())?;
        if segment.contains('\0') || segment.contains('\\') || segment.eq_ignore_ascii_case("compose") { return Err("owner has rejected cross-platform path component".to_string()); }
        segments.push(segment);
    }
    if segments.is_empty() { return Err("owner is workspace root".to_string()); }
    Ok(segments.join("/"))
}

#[cfg(test)]
mod mutation_source_authority_tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-source-authority/🧫️fixtures/🔣️cases.json")).unwrap() }

    fn link_file(target: &Path, link: &Path) {
        #[cfg(unix)] std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)] std::os::windows::fs::symlink_file(target, link).unwrap();
    }

    fn link_dir(target: &Path, link: &Path) {
        #[cfg(unix)] std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)] std::os::windows::fs::symlink_dir(target, link).unwrap();
    }

    fn fixture_workspace(case: &str) -> PathBuf {
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).unwrap();
        fs::canonicalize(base).unwrap().join(format!("semio-source-authority-{case}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    fn materialize(case: &str, fixture: &serde_json::Value) -> (PathBuf, PathBuf, PathBuf) {
        let workspace = fixture_workspace(case);
        let mutation_root = workspace.join("domain/🧬️mutations");
        let owner = mutation_root.join("🆕️insert-page");
        let source = owner.join("🦀️.rs");
        let descriptor = owner.join("🔣️.json");
        let taxonomy = workspace.join("authority/🔣️taxonomy.json");
        fs::create_dir_all(&owner).unwrap();
        fs::create_dir_all(taxonomy.parent().unwrap()).unwrap();
        fs::write(workspace.join("nx.json"), "{}").unwrap();
        fs::write(&source, "pub struct Probe;").unwrap();
        let owner_text = mutation_authority_relative(&workspace, &owner).unwrap();
        let mut descriptor_value = fixture["descriptor"].clone();
        descriptor_value["owner"] = serde_json::Value::String(owner_text);
        fs::write(&descriptor, serde_json::to_vec(&descriptor_value).unwrap()).unwrap();
        fs::write(&taxonomy, r#"{"fileKinds":{"rust":{"emoji":"🦀️","extensionChains":[".rs"]},"json":{"emoji":"🔣️","extensionChains":[".json"]}},"mutationComponentFileKindId":"rust","mutationDescriptorFileKindId":"json","semanticCollections":{"🧬️mutations":{"kind":"mutation"}}}"#).unwrap();
        fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{"taxonomy":"authority/🔣️taxonomy.json"}}}"#).unwrap();
        match case {
            "missing-locator" => fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{}}}"#).unwrap(),
            "malformed-locator" => fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{"taxonomy":"../authority/🔣️taxonomy.json"}}}"#).unwrap(),
            "wrong-root-pair" => fs::remove_file(workspace.join("nx.json")).unwrap(),
            "wrong-primary-filename" => { let wrong = owner.join("🦀️component.rs"); fs::rename(&source, &wrong).unwrap(); return (workspace.clone(), workspace, wrong); },
            "owner-mismatch" => fs::write(&descriptor, r#"{"owner":"other/🧬️mutations/🆕️insert-page"}"#).unwrap(),
            "symlink-parent" => { let actual = mutation_root.join("actual-owner"); fs::rename(&owner, &actual).unwrap(); link_dir(&actual, &owner); },
            "symlink-source" => { let actual = owner.join("🦀️actual.rs"); fs::rename(&source, &actual).unwrap(); link_file(&actual, &source); },
            "symlink-descriptor" => { let actual = owner.join("🔣️actual.json"); fs::rename(&descriptor, &actual).unwrap(); link_file(&actual, &descriptor); },
            "symlink-taxonomy" => { let actual = workspace.join("authority/🔣️actual.json"); fs::rename(&taxonomy, &actual).unwrap(); link_file(&actual, &taxonomy); },
            "nested-nx-anchor" => { let nested = workspace.join("nested"); fs::create_dir_all(&nested).unwrap(); fs::write(nested.join("nx.json"), "{}").unwrap(); fs::rename(workspace.join("domain"), nested.join("domain")).unwrap(); return (workspace.clone(), workspace, nested.join("domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "symlink-ancestor" => { let link = workspace.join("domain-link"); link_dir(&workspace.join("domain"), &link); return (workspace.clone(), workspace, link.join("🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "symlink-parent-erasure" => { let link = workspace.join("erased"); link_dir(&workspace.join("domain"), &link); return (workspace.clone(), workspace, PathBuf::from("erased/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "virtual-compose" => return (workspace.clone(), workspace.clone(), workspace.join("compose/🧬️mutations/🆕️insert-page/🦀️.rs")),
            "raw-case-folded-compose-parent" => return (workspace.clone(), workspace, PathBuf::from("consumer/../COMPOSE/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")),
            "file-parent-erasure" => { fs::write(workspace.join("not-a-directory"), "file").unwrap(); return (workspace.clone(), workspace, PathBuf::from("not-a-directory/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "raw-nonutf8" => {
                #[cfg(unix)] { use std::os::unix::ffi::OsStringExt; return (workspace.clone(), workspace, PathBuf::from(std::ffi::OsString::from_vec(b"consumer/\xff".to_vec()))); }
                #[cfg(windows)] { use std::os::windows::ffi::OsStringExt; return (workspace.clone(), workspace, PathBuf::from(std::ffi::OsString::from_wide(&[0xD800]))); }
            },
            "valid-relative-parent" => { fs::create_dir_all(workspace.join("consumer")).unwrap(); return (workspace.clone(), workspace, PathBuf::from("consumer/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            _ => {},
        }
        (workspace.clone(), workspace, source)
    }

    #[test]
    fn validates_mutation_source_authority_fixture() {
        let fixture = fixture();
        assert_eq!(fixture["schemaVersion"], 1);
        let mut descriptor_keys: Vec<&str> = fixture["descriptor"].as_object().unwrap().keys().map(String::as_str).collect(); descriptor_keys.sort_unstable(); assert_eq!(descriptor_keys, ["aggregateVariant", "binaryTag", "composition", "diffParticipation", "displayName", "emoji", "invertibility", "outcomeClasses", "owner", "payloadSchema", "requiredLanguageSurfaces", "schemaVersion", "semanticKind", "textOpcode"]);
        for vector in fixture["cases"].as_array().unwrap() {
            let name = vector["name"].as_str().unwrap();
            let (workspace, compiler_cwd, source) = materialize(name, &fixture);
            let result = mutation_source_authority(&source, &compiler_cwd);
            assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{name}: {result:?}");
            #[cfg(any(unix, windows))] if name == "raw-nonutf8" { assert!(result.as_ref().unwrap_err().contains("not UTF-8")); }
            if name == "nested-nx-anchor" { assert!(result.as_ref().unwrap_err().contains("lacks paired")); }
            if name == "file-parent-erasure" { assert_eq!(fs::metadata(compiler_cwd.join(&source)).unwrap_err().kind(), std::io::ErrorKind::NotADirectory); }
            if let Ok(facts) = result { assert_eq!(facts.workspace_root, workspace); assert!(facts.mutation_root.ends_with("domain/🧬️mutations")); assert!(facts.owner.ends_with("🆕️insert-page")); assert_eq!(facts.source_path.file_name().and_then(|name| name.to_str()), Some("🦀️.rs")); assert_eq!(facts.descriptor_path.file_name().and_then(|name| name.to_str()), Some("🔣️.json")); assert!(facts.taxonomy_path.ends_with("authority/🔣️taxonomy.json")); }
        }
    }
}
//#endregion 🔖️MutationSourceAuthority
fn main() {
let arguments: Vec<String> = std::env::args().collect();
let result = mutation_source_authority(Path::new(&arguments[1]), Path::new(&arguments[2]));
let output = match result {
Ok(facts) => serde_json::json!({"accepted": true, "workspace": facts.workspace_root.display().to_string(), "owner": facts.owner}),
Err(error) => serde_json::json!({"accepted": false, "error": error}),
};
println!("{}", output);
}