#[path = "📜️original-media-ingress.rs"]
mod ingress;

use ingress::{OriginalMediaFrame, OriginalMediaLoan, OriginalMediaSelection, ORIGINAL_MEDIA_PAGE_BYTES, send_original_media};
use std::cell::Cell;
use std::future::Future;
use std::io::{self, Cursor, Read};
use std::pin::pin;
use std::task::{Context, Poll, Waker};

struct MeasuredSource(Cursor<Vec<u8>>);

impl Read for MeasuredSource {
    fn read(&mut self, destination: &mut [u8]) -> io::Result<usize> {
        assert!(destination.len() <= ORIGINAL_MEDIA_PAGE_BYTES);
        self.0.read(destination)
    }
}

fn ready<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("native fixture does not need an asynchronous I/O worker"),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let operation = u64::from_str_radix(&args.next().unwrap(), 16).unwrap();
    let generation = u64::from_str_radix(&args.next().unwrap(), 16).unwrap();
    let instance = args.next().unwrap().parse().unwrap();
    let document = args.next().unwrap();
    let revision = args.next().unwrap();
    let revision = std::array::from_fn(|i| u8::from_str_radix(&revision[2*i..2*i+2], 16).unwrap());
    let scenario = args.next().unwrap();
    let size = args.next().unwrap().parse::<usize>().unwrap();
    let limit = args.next().unwrap().parse().unwrap();
    let trigger = args.next().unwrap().parse::<usize>().ok();
    let acknowledged = args.next().unwrap().parse::<usize>().unwrap();
    let expected = args.next().unwrap();
    let filename = args.next().unwrap();
    let media = args.next().unwrap();
    assert!(args.next().is_none());
    let loan = OriginalMediaLoan { operation_id: operation, generation, instance_id: instance, document_id: &document, base_revision: &revision };
    let selection = OriginalMediaSelection { name: &filename, media_type: Some(&media), byte_length: size };
    let mut state = 0x3167ab52u32;
    let original: Vec<u8> = (0..size).map(|_| {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state as u8
    }).collect();
    let available = if scenario == "short-source-read" { ORIGINAL_MEDIA_PAGE_BYTES + 17 } else { size };
    let mut source = MeasuredSource(Cursor::new(original[..available].to_vec()));
    let current = Cell::new(true);
    let accepted = Cell::new(0);
    let offset = Cell::new(0);
    let mut terminal = "refused";
    let mut live = false;
    let outcome = ready(send_original_media(&mut source, loan, selection, limit, async |frame| {
        match frame {
            OriginalMediaFrame::Begin { loan: received, selection } => {
                assert_eq!(received, loan);
                assert_eq!(selection.byte_length, size);
                assert_eq!(selection.media_type, Some(if media.is_empty() { "application/octet-stream" } else { media.as_str() }));
                assert!(!live);
                live = true;
            }
            OriginalMediaFrame::Page { loan: received, index, bytes } => {
                assert_eq!(received, loan);
                assert!(live && !bytes.is_empty() && bytes.len() <= ORIGINAL_MEDIA_PAGE_BYTES);
                assert_eq!(index as usize, accepted.get());
                if scenario == "rejected-second-page" && trigger == Some(index as usize) {
                    return Err("test sink denied the page");
                }
                let start = offset.get();
                assert_eq!(bytes, &original[start..start+bytes.len()]);
                offset.set(start + bytes.len());
                accepted.set(accepted.get() + 1);
            }
            OriginalMediaFrame::Complete { loan: received } => {
                assert_eq!(received, loan);
                assert!(live && offset.get() == size);
                terminal = "complete";
                live = false;
            }
            OriginalMediaFrame::Cancel { loan: received } => {
                assert_eq!(received, loan);
                assert!(live);
                terminal = "cancel";
                live = false;
            }
        }
        Ok(())
    }, || current.get(), |completed, length| {
        assert_eq!((completed, length), (offset.get(), size));
        if matches!(scenario.as_str(), "cancel-after-first-page" | "revision-changed") && trigger == Some(accepted.get()) { current.set(false); }
    }));
    assert_eq!(outcome.is_ok(), expected == "complete", "{outcome:?}");
    assert!(!live);
    assert_eq!(accepted.get(), acknowledged);
    assert_eq!(terminal, expected);
    if expected == "refused" { assert_eq!(source.0.position(), 0); }
    println!("[DEBUG] {scenario} accepted {} page(s) and ended {terminal}", accepted.get());
}
