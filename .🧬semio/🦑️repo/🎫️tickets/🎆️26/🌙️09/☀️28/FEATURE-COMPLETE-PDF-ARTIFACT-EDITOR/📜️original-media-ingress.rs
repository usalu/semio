//! 📤️ Sends a native selected file to an issued instance loan one bounded page at a time.

use std::io::{self, Read};

pub const ORIGINAL_MEDIA_PAGE_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OriginalMediaLoan<'a> {
    pub operation_id: u64,
    pub generation: u64,
    pub instance_id: u32,
    pub document_id: &'a str,
    pub base_revision: &'a [u8; 32],
}

#[derive(Clone, Copy)]
pub struct OriginalMediaSelection<'a> {
    pub name: &'a str,
    pub media_type: Option<&'a str>,
    pub byte_length: usize,
}

pub enum OriginalMediaFrame<'a> {
    Begin { loan: OriginalMediaLoan<'a>, selection: OriginalMediaSelection<'a> },
    Page { loan: OriginalMediaLoan<'a>, index: u32, bytes: &'a [u8] },
    Complete { loan: OriginalMediaLoan<'a> },
    Cancel { loan: OriginalMediaLoan<'a> },
}

#[derive(Debug)]
pub enum OriginalMediaSendError<E> {
    StaleLoan,
    Extent,
    Metadata,
    Read(io::Error),
    Transport(E),
    Retirement { failure: Box<Self>, error: E },
}

pub(crate) fn media_type_valid(value: &str) -> bool {
    let valid_token = |piece: &str| {
        let mut bytes = piece.bytes();
        let alphanumeric = |byte: u8| byte.is_ascii_lowercase() || byte.is_ascii_digit();
        bytes.next().is_some_and(alphanumeric) && bytes.all(|byte| alphanumeric(byte) || matches!(byte, b'.' | b'+' | b'-'))
    };
    value.len() <= 255 && value.split_once('/').is_some_and(|(kind, subtype)| valid_token(kind) && valid_token(subtype))
}

/// 🪟️ Reads native bytes within an I/O worker and waits for the exact instance to acknowledge each page.
pub async fn send_original_media<E>(
    source: &mut impl Read,
    loan: OriginalMediaLoan<'_>,
    selection: OriginalMediaSelection<'_>,
    maximum_file_bytes: usize,
    mut sink: impl for<'a> AsyncFnMut(OriginalMediaFrame<'a>) -> Result<(), E>,
    mut current: impl FnMut() -> bool,
    mut progress: impl FnMut(usize, usize),
) -> Result<(), OriginalMediaSendError<E>> {
    use OriginalMediaFrame as Frame;
    use OriginalMediaSendError as Error;
    if !current() { return Err(Error::StaleLoan); }
    let total = selection.byte_length;
    if total == 0 || total > maximum_file_bytes || total > 512 * 1024 * 1024 {
        return Err(Error::Extent);
    }
    let media_type = selection.media_type.filter(|kind| !kind.is_empty()).unwrap_or("application/octet-stream");
    if selection.name.is_empty() || selection.name.chars().take(256).count() > 255 ||
       loan.document_id.is_empty() || loan.document_id.chars().take(257).count() > 256 || !media_type_valid(media_type) {
        return Err(Error::Metadata);
    }
    let selection = OriginalMediaSelection { media_type: Some(media_type), ..selection };
    let mut page = [0; ORIGINAL_MEDIA_PAGE_BYTES];
    let result = async {
        sink(Frame::Begin { loan, selection }).await.map_err(Error::Transport)?;
        let mut offset = 0;
        for index in 0..total.div_ceil(ORIGINAL_MEDIA_PAGE_BYTES) as u32 {
            if !current() { return Err(Error::StaleLoan); }
            let count = (total - offset).min(ORIGINAL_MEDIA_PAGE_BYTES);
            let bytes = &mut page[..count];
            source.read_exact(bytes).map_err(Error::Read)?;
            if !current() { return Err(Error::StaleLoan); }
            sink(Frame::Page { loan, index, bytes }).await.map_err(Error::Transport)?;
            offset += count;
            progress(offset, total);
        }
        if !current() { return Err(Error::StaleLoan); }
        let mut tail = [0];
        if source.read(&mut tail).map_err(Error::Read)? != 0 {
            return Err(Error::Read(io::Error::new(io::ErrorKind::InvalidData, "The selected media grew during the read")));
        }
        if !current() { return Err(Error::StaleLoan); }
        sink(Frame::Complete { loan }).await.map_err(Error::Transport)
    }.await;
    if let Err(failure) = result {
        return match sink(Frame::Cancel { loan }).await {
            Ok(()) => Err(failure),
            Err(error) => Err(Error::Retirement { failure: Box::new(failure), error }),
        };
    }
    Ok(())
}
