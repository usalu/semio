//! Async facades for [`crate::kit::KitStore`]: no lock held across `.await`.
#![allow(dead_code)]

use futures_lite::future::ready;

use crate::diff::{DesignChange, DesignDiff};
use crate::error::{Result, SemioError};
use crate::kit::{KitStore, KitStoreRef};
use crate::report::{SemioReport, ValidationResult};

impl KitStore {
    pub async fn set_name_async(this: &KitStoreRef, name: String) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_name(name);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_description_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_description(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_icon_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_icon(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_image_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_image(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_preview_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_preview(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_version_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_version(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_remote_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_remote(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_homepage_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_homepage(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_license_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_license(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_uri_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_uri(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_created_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_created(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn set_updated_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.set_updated(v);
            Ok(())
        })();
        ready(r).await
    }

    pub async fn hash_async(this: &KitStoreRef) -> Result<String> {
        let r = match this.read() {
            Ok(g) => Ok(g.hash()),
            Err(_) => Err(SemioError::LockPoisoned("kit")),
        };
        ready(r).await
    }

    pub async fn validate_async(this: &KitStoreRef) -> Result<ValidationResult> {
        let r = match this.read() {
            Ok(g) => Ok(g.validate()),
            Err(_) => Err(SemioError::LockPoisoned("kit")),
        };
        ready(r).await
    }

    pub async fn apply_design_diff_async(this: &KitStoreRef, design_guid: &str, diff: &DesignDiff) -> Result<()> {
        let r = (|| {
            let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            g.apply_design_diff(design_guid, diff)
        })();
        ready(r).await
    }

    pub async fn flatten_design_async(this: &KitStoreRef, design_guid: &str) -> Result<SemioReport<DesignChange>> {
        let r = match this.read() {
            Ok(g) => g.flatten_design(design_guid),
            Err(_) => Err(SemioError::LockPoisoned("kit")),
        };
        ready(r).await
    }
}
