use super::*;
use std::io::{Cursor, Read, Write};

/// 🧫️ The language-neutral cases (`🧫️fixtures/🎒️zip-archive-cases/🔣️.json`): CRC-32 vectors, archives as named text
/// entries, and hostile mutations with the error each must raise.
#[derive(serde::Deserialize)]
struct Cases {
    crc32: Vec<CrcCase>,
    archives: Vec<ArchiveCase>,
    hostile: Vec<HostileCase>,
}

#[derive(serde::Deserialize)]
struct CrcCase {
    input: String,
    crc: String,
}

#[derive(serde::Deserialize)]
struct ArchiveCase {
    name: String,
    entries: Vec<EntryCase>,
}

#[derive(serde::Deserialize)]
struct EntryCase {
    name: String,
    text: String,
}

#[derive(serde::Deserialize)]
struct HostileCase {
    name: String,
    mutation: String,
    error: String,
}

fn cases() -> Cases {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🎒️zip-archive-cases/🔣️.json")).expect("zip archive cases parse")
}

fn ours(entries: &[EntryCase]) -> Vec<u8> {
    let mut writer = ZipWriter::new();
    for entry in entries {
        writer.add(&entry.name, entry.text.as_bytes()).expect("entry fits");
    }
    writer.finish().expect("archive fits")
}

/// 🧮️ The CRC-32 vectors of the fixture, including the standard check value of `123456789`.
#[test]
fn crc32_matches_the_fixture_vectors() {
    for case in cases().crc32 {
        assert_eq!(format!("{:08x}", crc32(case.input.as_bytes())), case.crc, "crc32({:?})", case.input);
    }
}

/// 🔁️ Every fixture archive written by this module reads back entry for entry, in order, through this module.
#[test]
fn own_archives_round_trip_in_order() {
    for archive in cases().archives {
        let bytes = ours(&archive.entries);
        let parsed = ZipArchive::parse(&bytes).unwrap_or_else(|error| panic!("{}: {error}", archive.name));
        assert_eq!(parsed.entries().iter().map(|entry| entry.name.as_str()).collect::<Vec<_>>(), archive.entries.iter().map(|entry| entry.name.as_str()).collect::<Vec<_>>(), "{}", archive.name);
        for entry in &archive.entries {
            assert_eq!(parsed.read(&entry.name, entry.text.len()).expect("reads"), entry.text.as_bytes(), "{}: {}", archive.name, entry.name);
        }
        assert_eq!(ours(&archive.entries), bytes, "{}: the writer is deterministic", archive.name);
    }
}

/// 🔮️ Oracle, one direction: the third-party `zip` crate reads every archive this module writes.
#[test]
fn the_zip_crate_reads_our_archives() {
    for archive in cases().archives {
        let bytes = ours(&archive.entries);
        let mut oracle = zip::ZipArchive::new(Cursor::new(bytes)).unwrap_or_else(|error| panic!("{}: {error}", archive.name));
        assert_eq!(oracle.len(), archive.entries.len(), "{}", archive.name);
        for entry in &archive.entries {
            let mut read = Vec::new();
            oracle.by_name(&entry.name).expect("oracle finds the entry").read_to_end(&mut read).expect("oracle inflates the entry");
            assert_eq!(read, entry.text.as_bytes(), "{}: {}", archive.name, entry.name);
        }
    }
}

/// 🔮️ Oracle, other direction: this module reads what the `zip` crate writes (deflated and stored), so packages
/// published before this module existed keep opening.
#[test]
fn we_read_the_zip_crate_archives() {
    for archive in cases().archives {
        for method in [zip::CompressionMethod::Deflated, zip::CompressionMethod::Stored] {
            let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
            let options = zip::write::SimpleFileOptions::default().compression_method(method);
            for entry in &archive.entries {
                writer.start_file(entry.name.as_str(), options).expect("oracle starts the entry");
                writer.write_all(entry.text.as_bytes()).expect("oracle writes the entry");
            }
            let bytes = writer.finish().expect("oracle finishes").into_inner();
            let parsed = ZipArchive::parse(&bytes).unwrap_or_else(|error| panic!("{} {method:?}: {error}", archive.name));
            for entry in &archive.entries {
                assert_eq!(parsed.read(&entry.name, entry.text.len()).expect("reads the oracle's entry"), entry.text.as_bytes(), "{} {method:?}: {}", archive.name, entry.name);
            }
        }
    }
}

/// 🚧️ Hostile archives raise the fixture's typed error instead of garbage or a panic.
#[test]
fn hostile_archives_raise_their_typed_error() {
    let fixture = cases();
    let base = ours(&fixture.archives[0].entries);
    let central = u32::from_le_bytes(base[base.len() - 6..base.len() - 2].try_into().expect("four bytes")) as usize;
    for case in fixture.hostile {
        let mut bytes = base.clone();
        match case.mutation.as_str() {
            "truncate-to-21" => bytes.truncate(21),
            "zero-last-22" => {
                let at = bytes.len() - 22;
                bytes[at..].fill(0);
            }
            "flip-central-crc" => bytes[central + 16] ^= 0xFF,
            "entry-count-ffff" => {
                let at = bytes.len() - 12;
                bytes[at..at + 2].copy_from_slice(&u16::MAX.to_le_bytes());
            }
            "set-central-encryption-flag" => bytes[central + 8] |= 1,
            other => panic!("unknown mutation {other}"),
        }
        let error = ZipArchive::parse(&bytes).and_then(|parsed| {
            let entries = parsed.entries().to_vec();
            entries.iter().try_for_each(|entry| parsed.read_entry(entry, usize::MAX).map(drop))
        });
        let kind = match error {
            Err(ZipArchiveError::Truncated(_)) => "Truncated",
            Err(ZipArchiveError::Signature(_)) => "Signature",
            Err(ZipArchiveError::Unsupported(_)) => "Unsupported",
            Err(ZipArchiveError::Crc(_)) => "Crc",
            Err(other) => panic!("{}: unexpected {other}", case.name),
            Ok(()) => panic!("{}: accepted", case.name),
        };
        assert_eq!(kind, case.error, "{}", case.name);
    }
}

/// 📏️ An entry larger than the caller's bound is refused before it is inflated.
#[test]
fn an_entry_beyond_the_bound_is_refused() {
    let bytes = ours(&[EntryCase { name: "big".into(), text: "x".repeat(4096) }]);
    assert_eq!(ZipArchive::parse(&bytes).expect("parses").read("big", 4095), Err(ZipArchiveError::TooLarge("big".into())));
}
