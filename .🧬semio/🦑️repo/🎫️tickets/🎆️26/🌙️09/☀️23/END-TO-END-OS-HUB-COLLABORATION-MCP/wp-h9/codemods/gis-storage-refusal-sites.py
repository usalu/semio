"""Routes every GIS Map approval `Storage` construction through one #[track_caller] recorder."""
import re
p = "/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs"
s = open(p, encoding="utf-8").read()

old_helper = s[s.index("#[cfg(test)]\nstatic LAST_GIS_MAP_STORAGE_REFUSAL"):s.index("/// 🔬️ The last `Storage` refusal of a retained approval in this process.")]
new_helper = '''#[cfg(test)]
static LAST_GIS_MAP_STORAGE_REFUSAL: Mutex<Option<String>> = Mutex::new(None);

/// 🧾️ The one constructor of a retained approval's `Storage` refusal. It records the exact source
/// line that refused and the authority's own error, so a law that only sees `Err(Storage)` learns
/// where and why without a second diagnostic channel.
#[track_caller]
fn gis_map_storage_refusal(error: &dyn std::fmt::Debug) -> GisMapApprovalCommitErrorV1 {
    #[cfg(test)]
    {
        *LAST_GIS_MAP_STORAGE_REFUSAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(format!("{}: {error:?}", std::panic::Location::caller()));
    }
    let _ = error;
    GisMapApprovalCommitErrorV1::Storage
}

'''
s = s.replace(old_helper, new_helper)

unrecovered_old = '''            unrecovered => {
                if let Ok((_, None)) = unrecovered {
                    record_gis_map_storage_refusal("recovery owners", &"a completed recovery scan returned no terminal owners");
                }
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                Err(GisMapApprovalCommitErrorV1::Storage)
            }'''
unrecovered_new = '''            Ok((_, None)) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                Err(gis_map_storage_refusal(&"a completed recovery scan returned no terminal owners"))
            }
            Err(error) => {
                documents.insert(key.to_owned(), RetainedGisMapDocumentStateV1::Recovery { owner, handle, scope, generation, document_write, fence });
                Err(error)
            }'''
assert s.count(unrecovered_old) == 1
s = s.replace(unrecovered_old, unrecovered_new)

rules = [
    (r'\.map_err\(\|(\w+)\| \{\s*record_gis_map_storage_refusal\("[^"]*", &(\w+)\);\s*GisMapApprovalCommitErrorV1::Storage\s*\}\)', r'.map_err(|\1| gis_map_storage_refusal(&\2))'),
    (r'record_gis_map_storage_refusal\("[^"]*", &(\w+)\);\s*return Err\(GisMapApprovalCommitErrorV1::Storage\);', r'return Err(gis_map_storage_refusal(&\1));'),
    (r'(\w+) => \{\s*record_gis_map_storage_refusal\("[^"]*", &\w+\);\s*Err\(GisMapApprovalCommitErrorV1::Storage\)\s*\}', r'\1 => Err(gis_map_storage_refusal(&\1)),'),
    (r'\.map_err\(\|_\| GisMapApprovalCommitErrorV1::Storage\)', r'.map_err(|error| gis_map_storage_refusal(&error))'),
    (r'\.ok_or\(GisMapApprovalCommitErrorV1::Storage\)', r'.ok_or_else(|| gis_map_storage_refusal(&"absent"))'),
    (r'_ => GisMapApprovalCommitErrorV1::Storage,', r'other => gis_map_storage_refusal(&other),'),
    (r'_ => Err\(GisMapApprovalCommitErrorV1::Storage\),', r'phase => Err(gis_map_storage_refusal(&phase)),'),
    (r'Err\(_\) => Err\(GisMapApprovalCommitErrorV1::Storage\),', r'Err(_) => Err(gis_map_storage_refusal(&"checkpoint notification panicked")),'),
    (r'GisMapCommitTurnV1::Rejected\(GisMapApprovalCommitErrorV1::Storage\)', r'GisMapCommitTurnV1::Rejected(gis_map_storage_refusal(&"turn refused"))'),
    (r'Err\(GisMapApprovalCommitErrorV1::Storage\)', r'Err(gis_map_storage_refusal(&"refused"))'),
]
for pattern, replacement in rules:
    s, n = re.subn(pattern, replacement, s)
    print(n, pattern[:70])
assert "record_gis_map_storage_refusal" not in s, "a recorded site was left"
left = [line.strip() for line in s.splitlines() if "GisMapApprovalCommitErrorV1::Storage" in line]
print(left)
open(p, "w", encoding="utf-8").write(s)
