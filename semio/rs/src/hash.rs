use sha2::{Digest, Sha256};

/// Stable content-hash writer that feeds `sha2::Sha256` with deterministic
/// string primitives. Used by domain entities to produce their own canonical
/// hash fingerprint on demand.
pub struct HashWriter {
    inner: Sha256,
}

impl HashWriter {
    pub fn new() -> Self {
        Self { inner: Sha256::new() }
    }

    pub fn tag(&mut self, tag: &str) -> &mut Self {
        self.inner.update(tag.as_bytes());
        self.inner.update(b"\0");
        self
    }

    pub fn str(&mut self, s: &str) -> &mut Self {
        self.inner.update((s.len() as u64).to_le_bytes());
        self.inner.update(s.as_bytes());
        self
    }

    pub fn opt_str(&mut self, s: Option<&str>) -> &mut Self {
        match s {
            Some(v) => {
                self.inner.update(b"S");
                self.str(v);
            }
            None => {
                self.inner.update(b"N");
            }
        }
        self
    }

    pub fn f64(&mut self, v: f64) -> &mut Self {
        self.inner.update(v.to_le_bytes());
        self
    }

    pub fn opt_f64(&mut self, v: Option<f64>) -> &mut Self {
        match v {
            Some(x) => {
                self.inner.update(b"F");
                self.f64(x);
            }
            None => {
                self.inner.update(b"N");
            }
        }
        self
    }

    pub fn bool(&mut self, v: bool) -> &mut Self {
        self.inner.update([if v { 1u8 } else { 0u8 }]);
        self
    }

    pub fn opt_bool(&mut self, v: Option<bool>) -> &mut Self {
        match v {
            Some(x) => {
                self.inner.update(b"B");
                self.bool(x);
            }
            None => {
                self.inner.update(b"N");
            }
        }
        self
    }

    pub fn finalize(self) -> String {
        hex::encode(self.inner.finalize())
    }
}

impl Default for HashWriter {
    fn default() -> Self {
        Self::new()
    }
}
