//! ZIP bundle: `kit.json` (pretty JSON) at archive root. Asset files may be
//! added in a follow-up; hosts should prefer JSON round-trip for fidelity today.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::error::{Result, SemioError};
use crate::kit::{KitStore, KitStoreRef};

const KIT_JSON: &str = "kit.json";

impl KitStore {
    /// Preferred API (plan): write `kit.json` into a zip at `path`.
    pub fn save_zip(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut zip = ZipWriter::new(file);
        let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        zip.start_file(KIT_JSON, opts)?;
        zip.write_all(self.to_json_pretty()?.as_bytes())?;
        zip.finish()?;
        Ok(())
    }

    /// Preferred API (plan): read `kit.json` from a zip at `path`.
    pub fn load_zip(path: &Path) -> Result<KitStoreRef> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut kit_json = String::new();
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            if entry.name() == KIT_JSON {
                entry.read_to_string(&mut kit_json)?;
                break;
            }
        }
        if kit_json.is_empty() {
            return Err(SemioError::InvalidOperation(format!("zip missing {KIT_JSON}")));
        }
        KitStore::from_json_str(&kit_json)
    }

    pub fn from_zip(path: &Path) -> Result<KitStoreRef> {
        Self::load_zip(path)
    }

    pub fn to_zip(&self, path: &Path) -> Result<()> {
        self.save_zip(path)
    }
}
