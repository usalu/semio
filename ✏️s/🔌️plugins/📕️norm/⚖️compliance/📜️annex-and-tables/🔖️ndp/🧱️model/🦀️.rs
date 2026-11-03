//! 🧱️ Row model of the NDP registry: the value, source, status and row types plus the compact constructors the per-Eurocode table crates use. Leaf of the registry; lookups live in `semio-s-plugin-norm-ndp`.

/// 🛡️ Whether an NDP value was double-keyed from the printed standard; only `Verified` may support a compliant verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NdpStatus {
    Verified,
    Unverified,
}

/// 🔢️ An NDP value as written in the code: a number, a table or formula text, or not yet transcribed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NdpValue {
    Unset,
    Number(f64),
    Text(&'static str),
}

/// 📚️ Where a value comes from: standard, print edition and table or clause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NdpSource {
    pub document: &'static str,
    pub edition: &'static str,
    pub locator: &'static str,
}

/// 🧾️ One NDP or table cell; `status` is `Verified` only when `status_en` and `status_de` are, because the value of each annex is double-keyed on its own; `id` is the stable key checks cite via [`CheckBuilder::ndp`]. The audit provenance (`specRef`, `asWritten`, `evidence`) lives in `🔣️.json` only, which keeps this runtime table compact.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NdpEntry {
    pub id: &'static str,
    pub code: &'static str,
    pub clause: &'static str,
    pub symbol: &'static str,
    pub recommended: NdpValue,
    pub na_de: NdpValue,
    pub unit: &'static str,
    pub source: NdpSource,
    pub status: NdpStatus,
    pub status_en: NdpStatus,
    pub status_de: NdpStatus,
}

const PENDING: &str = "to be recorded at verification";

/// 🧱️ Builds a row whose EN-recommended and NA-DE values agree; locator is the clause, edition pending, status unverified (see [`NdpEntry::at`], [`NdpEntry::ed`], [`NdpEntry::ver`]); keeps the generated table compact.
pub const fn c(id: &'static str, code: &'static str, clause: &'static str, symbol: &'static str, value: NdpValue, unit: &'static str, document: &'static str) -> NdpEntry {
    d(id, code, clause, symbol, value, value, unit, document)
}

/// 🧱️ Builds a row with distinct EN-recommended and NA-DE values; same defaults as [`c`].
#[allow(clippy::too_many_arguments)]
pub const fn d(id: &'static str, code: &'static str, clause: &'static str, symbol: &'static str, recommended: NdpValue, na_de: NdpValue, unit: &'static str, document: &'static str) -> NdpEntry {
    NdpEntry { id, code, clause, symbol, recommended, na_de, unit, source: NdpSource { document, edition: PENDING, locator: clause }, status: NdpStatus::Unverified, status_en: NdpStatus::Unverified, status_de: NdpStatus::Unverified }
}

impl NdpEntry {
    /// 📍️ Replaces the locator.
    pub const fn at(mut self, locator: &'static str) -> Self {
        self.source.locator = locator;
        self
    }

    /// 📅️ Records the print edition.
    pub const fn ed(mut self, edition: &'static str) -> Self {
        self.source.edition = edition;
        self
    }

    /// ✅️ Marks the row as verified against the print for every annex.
    pub const fn ver(mut self) -> Self {
        self.status = NdpStatus::Verified;
        self.status_en = NdpStatus::Verified;
        self.status_de = NdpStatus::Verified;
        self
    }

    /// ✅️ Marks the EN-recommended value as verified; the row status follows once the NA-DE value is verified too.
    pub const fn ver_en(mut self) -> Self {
        self.status_en = NdpStatus::Verified;
        self.status = self.joint();
        self
    }

    /// ✅️ Marks the NA-DE value as verified; the row status follows once the EN-recommended value is verified too.
    pub const fn ver_de(mut self) -> Self {
        self.status_de = NdpStatus::Verified;
        self.status = self.joint();
        self
    }

    const fn joint(&self) -> NdpStatus {
        match (self.status_en, self.status_de) {
            (NdpStatus::Verified, NdpStatus::Verified) => NdpStatus::Verified,
            _ => NdpStatus::Unverified,
        }
    }
}
