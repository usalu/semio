//! ⚠️ Canonical container and schema refusal identities below document composition.

use semio_framework_diagnostic::{FaultOrigin, TextError};
use semio_framework_value::ValueError;

//#region 🔖️Errors
/// 🚨️ The one error type every `pack_*` public fn returns; never leaks `std::io::Error`.
#[derive(Debug, Clone, PartialEq)]
pub enum PackError {
    BadMagic,
    UnsupportedVersion { major: u16, minor: u16 },
    UnknownRequiredFlags(u32),
    Truncated(u64),
    ChecksumMismatch { segment: &'static str, offset: u64 },
    ContentHashMismatch,
    LimitExceeded(&'static str),
    RetainedMalformed { what: &'static str, offset: u64, detail: &'static str },
    Malformed { what: &'static str, offset: u64, detail: String },
    NonCanonical(&'static str),
    UnsupportedCodec(u8),
    ValueRefusal(ValueError),
    TextRefusal(TextError),
    Io(String),
}

impl std::fmt::Display for PackError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadMagic => formatter.write_str("bad magic"),
            Self::UnsupportedVersion { major, minor } => write!(formatter, "unsupported version {major}.{minor}"),
            Self::UnknownRequiredFlags(flags) => write!(formatter, "unknown required feature bits {flags:#x}"),
            Self::Truncated(offset) => write!(formatter, "truncated at offset {offset}"),
            Self::ChecksumMismatch { segment, offset } => write!(formatter, "checksum mismatch in {segment} at offset {offset}"),
            Self::ContentHashMismatch => formatter.write_str("content hash mismatch"),
            Self::LimitExceeded(limit) => write!(formatter, "limit exceeded: {limit}"),
            Self::RetainedMalformed { what, offset, detail } => write!(formatter, "malformed {what} at offset {offset}: {detail}"),
            Self::Malformed { what, offset, detail } => write!(formatter, "malformed {what} at offset {offset}: {detail}"),
            Self::NonCanonical(detail) => write!(formatter, "non-canonical encoding: {detail}"),
            Self::UnsupportedCodec(codec) => write!(formatter, "unsupported codec {codec}"),
            Self::ValueRefusal(message) => write!(formatter, "schema error: {message}"),
            Self::TextRefusal(error) => write!(formatter, "schema error: {error}"),
            Self::Io(message) => write!(formatter, "io error: {message}"),
        }
    }
}

impl std::error::Error for PackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self { Self::ValueRefusal(error) => Some(error), Self::TextRefusal(error) => Some(error), _ => None }
    }
}

impl From<ValueError> for PackError {
    fn from(error: ValueError) -> Self { Self::ValueRefusal(error) }
}

impl From<TextError> for PackError {
    fn from(error: TextError) -> Self { Self::TextRefusal(error) }
}

semio_framework_diagnostic::fault_from_error!(PackError, FaultOrigin::Module, "module.pack");

//#endregion 🔖️Errors
