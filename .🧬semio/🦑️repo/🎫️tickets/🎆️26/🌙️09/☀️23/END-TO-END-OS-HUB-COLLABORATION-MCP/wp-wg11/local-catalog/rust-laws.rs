/// 🗂️ The shared local-catalog vocabulary (`🏛️ShellHost/🗂️local-catalog/🔣️.json`) names exactly the notices this shell's lane
/// speaks, each in both tongues (any other locale reads English) under the code React's lane uses, and the lane's two commands.
#[test]
fn the_shared_local_catalog_vocabulary_speaks_every_notice_in_both_tongues() {
    let vocabulary = &*LOCAL_CATALOG_V1;
    let notices: std::collections::BTreeSet<String> = LocalCatalogNoticeV1::ALL.iter().map(|notice| notice.key()).collect();
    assert_eq!(vocabulary.notices.keys().cloned().collect::<std::collections::BTreeSet<_>>(), notices, "vocabulary keys == the lane's notices");
    for notice in LocalCatalogNoticeV1::ALL {
        let (en, de, fr) = (notice.text("en", "Plan"), notice.text("de", "Plan"), notice.text("fr", "Plan"));
        assert!(!en.is_empty() && !de.is_empty() && en != de && fr == en && !en.contains("{name}") && !de.contains("{name}"), "{notice:?}");
        assert_eq!(notice.code(), format!("shell.localCatalog.{}", notice.key()));
    }
    assert_eq!((vocabulary.actions.admit.as_str(), vocabulary.actions.retire.as_str()), ("os.local-catalog.admit", "os.local-catalog.retire"));
}

fn local_catalog_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// 🗂️ Every admission vector — the ones React's engine contract answers with ShellHost's lane — resolves here to the same
/// refusal, or to the same catalog entry, event folder and archive bytes.
#[test]
fn local_catalog_admissions_answer_the_shared_vectors() {
    assert!(!LOCAL_CATALOG_V1.admissions.is_empty());
    for vector in &LOCAL_CATALOG_V1.admissions {
        let actual = local_catalog_admission_v1(vector.args.as_ref(), vector.data_dir.as_deref(), vector.now_ms);
        match (&actual, vector.expect.refusal) {
            (Err(notice), Some(expected)) => assert_eq!(*notice, expected, "{}", vector.name),
            (Ok(admission), None) => {
                assert_eq!((Some(&admission.document), Some(&admission.folder), Some(local_catalog_hex(&admission.archive))), (vector.expect.document.as_ref(), vector.expect.folder.as_ref(), vector.expect.archive_hex.clone()), "{}", vector.name)
            }
            _ => panic!("{}: {actual:?} vs {:?}", vector.name, vector.expect),
        }
    }
}

/// 🗃️ The catalog facet persists as exactly the shared archive bytes — each `admittedAtMs` an exact `uint`, the bytes React's
/// lane writes — and reads back as itself.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_local_catalog_persists_as_the_shared_archive_bytes() {
    assert!(!LOCAL_CATALOG_V1.catalog_archives.is_empty());
    for vector in &LOCAL_CATALOG_V1.catalog_archives {
        let archive = local_catalog_archive_v1(&vector.catalog).expect("the catalog encodes");
        assert_eq!(local_catalog_hex(&archive), vector.archive_hex, "{:?}", vector.catalog);
        assert_eq!(drive(decode_local_catalog_archive_v1(&archive)).as_ref(), Some(&vector.catalog));
    }
}

/// 🗂️ The native lane end to end on a real data folder: an admission speaks while it runs, is written into its folder lane and
/// read back identically, is listed, and the catalog facet keeps it under `<dataDir>/os`, so a fresh shell on the same folder
/// lists it again; a retirement unlists it (its events stay) in the shell's tongue; a shell without a data folder, an unknown
/// retirement, an incomplete request and an admission whose session is gone before the write are refused out loud.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_kept_studio_is_written_listed_persisted_and_retired_on_this_device() {
    let data_dir = std::env::temp_dir().join(format!("semio-local-catalog-{}", semio_framework_os_kernel::os_identity::time_ordered_id()));
    let with_data = |locale: &str| {
        let mut shell = shell();
        shell.locale_id = locale.into();
        shell.identity_env = Some(IdentityEnv { data_dir: Some(data_dir.clone()) });
        shell.open_local_catalog();
        shell
    };
    let settle = |shell: &mut ShellState| {
        let started = std::time::Instant::now();
        while (shell.local_catalog.running.is_some() || !shell.local_catalog.jobs.is_empty()) && started.elapsed() < std::time::Duration::from_secs(20) {
            drive(shell.pump_local_catalog());
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    };
    let spoken = |shell: &ShellState| shell.transient_notice().map(|notice| (notice.message.clone(), notice.code.clone().unwrap_or_default()));
    let notice = |notice: LocalCatalogNoticeV1, locale: &str, name: &str| Some((notice.text(locale, name), notice.code()));
    let studio_lane = || store_sync::sync::FolderEventLogStorage::new(data_dir.join("os").join("local-documents").join("studio-1"));
    let studio = [("documentId", "studio-1"), ("schema", "s.space"), ("name", "Studio"), ("storage", "folder"), ("pack", "AQID"), ("spr", "BAU=")];

    let mut bare = shell();
    bare.locale_id = "en".into();
    shell_command(&mut bare, "os.local-catalog.admit", &studio);
    assert_eq!(spoken(&bare), notice(LocalCatalogNoticeV1::NoDataFolder, "en", ""), "no data folder, nothing kept");

    let mut a = with_data("en");
    settle(&mut a);
    shell_command(&mut a, "os.local-catalog.admit", &studio);
    assert_eq!(spoken(&a), notice(LocalCatalogNoticeV1::Keeping, "en", "Studio"), "the admission speaks while it runs");
    settle(&mut a);
    assert_eq!(spoken(&a), notice(LocalCatalogNoticeV1::Kept, "en", "Studio"));
    let kept = a.local_catalog_documents().to_vec();
    assert_eq!(kept.iter().map(|document| (document.document_id.as_str(), document.target.clone())).collect::<Vec<_>>(), vec![("studio-1", format!("{}/os/local-documents/studio-1", data_dir.to_string_lossy()))]);
    let written = drive(studio_lane().read_archive("studio-1")).expect("the studio lane reads").expect("the studio is on disk");
    assert_eq!(local_catalog_hex(&written), "010301020302040500", "the folder lane holds exactly the admitted archive");

    let mut b = with_data("de");
    settle(&mut b);
    assert_eq!(b.local_catalog_documents(), kept.as_slice(), "a fresh shell on the same data folder lists the kept studio");
    shell_command(&mut b, "os.local-catalog.retire", &[("documentId", "studio-1")]);
    settle(&mut b);
    assert_eq!(spoken(&b), notice(LocalCatalogNoticeV1::Retired, "de", "Studio"));
    assert!(b.local_catalog_documents().is_empty());
    assert!(drive(studio_lane().read_archive("studio-1")).expect("the studio lane reads").is_some(), "retiring unlists; the studio's events stay");
    shell_command(&mut b, "os.local-catalog.retire", &[("documentId", "studio-1")]);
    assert_eq!(spoken(&b), notice(LocalCatalogNoticeV1::DocumentUnknown, "de", ""));
    shell_command(&mut b, "os.local-catalog.admit", &[("documentId", "studio-2")]);
    assert_eq!(spoken(&b), notice(LocalCatalogNoticeV1::InvalidRequest, "de", ""));

    let mut c = with_data("en");
    settle(&mut c);
    assert!(c.local_catalog_documents().is_empty(), "the retirement persisted");
    let args = serde_json::json!({ "documentId": "studio-3", "schema": "s.space", "name": "Gone", "storage": "folder", "pack": "AQID", "spr": "" });
    let admission = local_catalog_admission_v1(Some(&args), Some(&data_dir.to_string_lossy()), 1).expect("a valid admission");
    c.local_catalog.jobs.push_back(LocalCatalogJobV1::Admit { requester: Some(7), admission: Box::new(admission) });
    settle(&mut c);
    assert_eq!(spoken(&c), notice(LocalCatalogNoticeV1::Cancelled, "en", "Gone"), "the asking session is gone before the write");
    assert!(c.local_catalog_documents().is_empty() && !data_dir.join("os").join("local-documents").join("studio-3").exists(), "nothing was written");
    let _ = std::fs::remove_dir_all(&data_dir);
}

